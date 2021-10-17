use crate::disc_objects::{
    Snowflake, ApplicationCommandType, AppCommandOptionType, ChannelType
};
use futures::future::{FutureExt, BoxFuture, Future};
use crate::disc_objects;
use crate::disc_objects::AppCommandValue;
use futures::lock::Mutex;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use crate::bot;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::bot::{BotClient, PinnedFuture};
use crate::discord;

pub type CommandType = ApplicationCommandType;
pub type PinnedInteraction = Box<dyn Fn(disc_objects::Interaction, bot::BotClient) -> BoxFuture<'static, ()> + Send + Sync>;
pub type AppCommandMap = Arc<RwLock<HashMap<disc_objects::ApplicationCommandType ,HashMap<String, InteractionCommand>>>>;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct AppCommand {
    pub id: Option<Snowflake>,

    #[serde(rename="type")]
    pub app_command_type: ApplicationCommandType,
    pub application_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub options: Vec<AppCommandOption>,
    pub default_permission: Option<bool>,
    pub version: Option<Snowflake>,

}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct AppCommandOption {

    #[serde(rename="type")]
    pub app_command_option_type: AppCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: Option<bool>,
    pub choices: Option<Vec<AppCommandChoice>>,
    pub options: Option<Vec<AppCommandOption>>,
    pub channel_types: Option<Vec<ChannelType>>

}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct AppCommandChoice {
    pub name: String,
    pub value: AppCommandValue,
}

impl AppCommandChoice {
    pub fn new(name: &str, value: AppCommandValue) -> Self {
        Self {
            name: String::from(name),
            value,
        }
    }
}

impl AppCommandOption {
    pub fn new(name: &str, description: &str, option_type: AppCommandOptionType) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            app_command_option_type: option_type,
            required: None,
            choices: None,
            options: None,
            channel_types: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = Some(!self.required.get_or_insert(false).clone());

        self
    }

    /// max choices is 25 :)
    pub fn add_choice(mut self, name: &str, value: AppCommandValue) -> Self {

        let value = match self.app_command_option_type {
            AppCommandOptionType::String | AppCommandOptionType::Integer | AppCommandOptionType::Number => value,
            _ => panic!("You can only add choice if option type is String, Integer or Number (double)")
        };

        let choice = AppCommandChoice::new(name, value);

        self.choices.get_or_insert(vec![]).push(choice);

        self
    }

    /// max choices is 25 :)
    pub fn add_choice_from_struct(mut self, choice: AppCommandChoice) -> Self {

        let choice = match self.app_command_option_type {
            AppCommandOptionType::String | AppCommandOptionType::Integer | AppCommandOptionType::Number => choice,
            _ => panic!("You can only add choice if option type is String, Integer or Number (double)")
        };

        self.choices.get_or_insert(vec![]).push(choice);

        self
    }

    pub fn add_channel_type(mut self, channel_type: ChannelType) -> Self {

        let channel_type = match self.app_command_option_type {
            AppCommandOptionType::Channel => channel_type,
            _ => panic!("You can only add channel types if the option type is channel!")
        };

        self.channel_types.get_or_insert(vec![]).push(channel_type);

        self
    }

    pub fn add_channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {

        let channel_types = match self.app_command_option_type {
            AppCommandOptionType::Channel => channel_types,
            _ => panic!("You can only add channel types if the option type is channel!")
        };

        self.channel_types.get_or_insert(vec![]).extend(channel_types);

        self
    }

    /// valid for only type of subcommand or subcommand group
    pub fn add_option(mut self, option: AppCommandOption) -> Self {

        let option = match self.app_command_option_type {
            AppCommandOptionType::SubCommand | AppCommandOptionType::SubCommandGroup => option,
            _ => panic!("You can only add options if option type is subcommand or subcommand group.")
        };

        self.options.get_or_insert(vec![]).push(option);

        self
    }

}

impl AppCommand {
    pub fn new(name : &str, description: &str, app_command_type: ApplicationCommandType) -> Self {
        Self {
            id: None,
            app_command_type,
            application_id: None,
            guild_id: None,
            name: String::from(name),
            description: Some(String::from(description)),

            options: vec![],
            default_permission: None,
            version: None
        }

    }

    pub fn add_option(mut self, option: AppCommandOption) -> Self {
        self.options.push(option);

        self
    }

}


pub struct InteractionCommand {
    pub app_command: AppCommand,
    pub guild_ids: Option<Vec<u64>>,
    pub function: PinnedInteraction,
}

impl InteractionCommand {
    pub fn new(app_command: AppCommand, function: PinnedInteraction, guild_ids: Option<Vec<u64>>) -> Self {
        Self {
            app_command,
            function,
            guild_ids,
        }

    }
}

impl crate::bot::Bot {
    pub async fn add_app_command(&mut self, command: AppCommand, function: PinnedInteraction, guild_ids: Vec<u64>) {
        let guild_ids = if guild_ids.len() == 0 {
            None
        } else {
            Some(guild_ids)
        };

        let interaction_command = InteractionCommand::new(
            command.clone(), function, guild_ids
        );

        let command_type = command.app_command_type.clone();
        let command_type_exists = self.interaction_map.read().await.get(&command_type).is_some();


        if command_type_exists {

            self.interaction_map.write().await.get_mut(&command_type).unwrap().insert(command.name, interaction_command);

        } else {

            let mut nested_map = HashMap::new();
            nested_map.insert(command.name, interaction_command);

            self.interaction_map.write().await.insert(command_type, nested_map);
        }
    }

    pub async fn create_command(client: bot::BotClient, guild_map: HashMap<Snowflake, Vec<Value>>) {

        let base = format!("/applications/{}/", client.lock().await.application_id);
        let global = String::from("global_commands");

        for (guild_id, app_commands) in guild_map {

            let mut extension = match guild_id {
                Snowflake::String(global) => String::from("commands"),
                Snowflake::Integer(guild_id) => format!("guilds/{}/commands", guild_id),
            };

            let overwrite = discord::HttpRequest::string_new(base.clone() + &extension, client.clone()).await;

            println!("{:#?}",overwrite.put(serde_json::Value::Array(app_commands)).await);

        }
    }

    pub async fn register_commands(client: bot::BotClient, interaction_map: AppCommandMap) {

        let mut guild_map = HashMap::new();

        for application_command_type in interaction_map.read().await.keys() {
            for (_name, command) in interaction_map.read().await.get(application_command_type).unwrap().iter() {

                let guild_ids = command.guild_ids.clone();

                let mut command = command.app_command.clone(); //but its mut here hmm why it clo

                command.description = match command.app_command_type {
                    ApplicationCommandType::User | ApplicationCommandType::Message => None,
                    ApplicationCommandType::ChatInput => command.description,
                };

                let command = serde_json::to_value(command.clone()).unwrap();

                if guild_ids.is_some() {
                    for guild_id in guild_ids.unwrap() {

                        let guild_id = Snowflake::Integer(guild_id);
                        guild_map.entry(guild_id).or_insert(vec![]).push(command.clone());
                    }
                } else {

                    let global_id = Snowflake::String(String::from("global_commands"));
                    guild_map.entry(global_id).or_insert(vec![]).push(command.clone());
                }
            };
        }

        tokio::spawn(async move {
            crate::bot::Bot::create_command(client.clone(), guild_map).await;
        });
    }

    pub async fn handle_interaction(interaction: Option<discord::GatewayEvent>, interaction_map: Option<AppCommandMap>, client: BotClient) {

        let interaction = match interaction {
            None => panic!("Did not receive any data along with interaction create event!!! discord what are you doing!?!??!"),
            Some(discord::GatewayEvent::InteractionCreate(interaction)) => interaction,
            _ => panic!("What even????"),
        };

        if interaction.data.is_some() {
            if interaction_map.is_some() {

                match interaction.interaction_type {
                    disc_objects::InteractionType::ApplicationCommand => bot::Bot::handle_app_command(interaction, interaction_map.unwrap(), client.clone()).await,
                    _ => panic!("Celestialcord only supports the application command type right now!"),
                }

            } else {
                println!("Did not receive interaction map on handle_interaction!")
            }
        }
    }

    pub async fn handle_app_command(interaction: disc_objects::Interaction, interaction_map: AppCommandMap, client: BotClient) {

        interaction_map.read().await.get(&interaction.clone().data.unwrap().interaction_data_type).unwrap().get(&interaction.clone().data.unwrap().name).unwrap().function.as_ref()(interaction, client.clone()).await;

    }
}