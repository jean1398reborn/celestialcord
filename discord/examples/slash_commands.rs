extern crate celestialcord;

use futures::future::{FutureExt};
use celestialcord::bot;
use celestialcord::discord;
use celestialcord::disc_objects;
use celestialcord::bot::{Bot};
use celestialcord::discord::{GatewayEvent, Intent};
use celestial_macros::add_fn;
use celestialcord::interactions;
use tokio::time::{Instant};
use celestialcord::disc_objects::{AppCommandOptionType, AppCommandValue};
use celestialcord::interactions::{AppCommandOption};

async fn on_ready(returned: discord::GatewayEvent, client: bot::BotClient) {
    println!("Bot ready!");
}

async fn responses(message : disc_objects::Interaction, client: bot::BotClient) {
    println!("ran response");
    let embed = disc_objects::Embed::new("Hello", "Burger man!", 0xFF0000)
        .image("https://c.tenor.com/zDUT9yR2Zz0AAAAC/big-buger-eat-buger.gif");

    let embed2 = disc_objects::Embed::new("Multiple embeds!", "With descriptions!!!", 0x0000FF);

    let mut reply = disc_objects::ReplyMessage::new(false)
        .add_embed(embed)
        .add_embed(embed2);

    let response = reply.callback_with_interaction(message.clone(), client.clone()).await;
}

async fn longtask(message : disc_objects::Interaction, client: bot::BotClient) {

    let mut test = message.clone();
    let hey = test.data.unwrap();
    let hoi = hey.options.unwrap();
    let hihi = hoi.get(0);
    let sup = hihi.unwrap().clone();
    let oi = sup.value.unwrap();

    let hi = match oi {
        AppCommandValue::String(sup) => sup,
        AppCommandValue::Integer(hoi) => hoi.to_string(),
        AppCommandValue::Double(bith) => bith.to_string(),
    };

    let mut reply = disc_objects::ReplyMessage::new(false)
        .content_str(format!("Task started, waiting {} seconds", hi).as_str())
        .callback_with_interaction(message.clone(), client.clone()).await;

    let time : u64 = hi.parse().unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(time)).await;

    let mut reply = disc_objects::ReplyMessage::new(false)
        .content_str("Task finished")
        .send_with_interaction(message.clone(), client.clone()).await;
}

#[tokio::main]
async fn main() {
    // make bot

    let mut bot = Bot::new(
        9,
        std::env::var("BOT_TOKEN").expect("Put bot token in env_var!"),
        vec![Intent::all()],
    );

    bot.sync_commands = true;

    // idk what its called but you make application commands like this~
    let response = interactions::AppCommand::new(
                                                     "response",
                                                 "Send a test response",
                                                 interactions::CommandType::ChatInput);


    bot.add_app_command(response, add_fn!(responses), vec![889400262224146492]).await;

    let seconds_option = AppCommandOption::new("seconds", "how many seconds to wait", AppCommandOptionType::Integer)
        .required()
        .add_choice("one whole second of waiting!!!!", AppCommandValue::Integer(1))
        .add_choice("three whole seconds of waiting!!!!", AppCommandValue::Integer(3))
        .add_choice("five whole seconds of waiting!!!!", AppCommandValue::Integer(5))
        .add_choice("ten whole second of waiting!!!!", AppCommandValue::Integer(10))
        .add_choice("sixty nine whole seconds of waiting!!!!", AppCommandValue::Integer(69));

    println!("{:#?}", seconds_option);
    let longtasks = interactions::AppCommand::new(
        "longtask",
        "Send a little command that waits for 10 seconds",
        interactions::CommandType::ChatInput)
        .add_option(seconds_option);


    bot.add_app_command(longtasks, add_fn!(longtask), vec![889400262224146492]).await;

    // add event system, where you add a function to be called to an event.
    bot.add_event(disc_objects::GatewayEventBinding::Ready, add_fn!(on_ready)).await;

    //alive
    bot.elevate().await;
}
