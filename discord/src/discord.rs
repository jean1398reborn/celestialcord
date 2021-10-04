use crate::{bot, disc_objects};
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use tokio::time::{sleep, Duration};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
type WbSS = WebSocketStream<MaybeTlsStream<TcpStream>>;

use serde_json::value::Value as SerdeValue;
use SerdeValue::Null as SerdeNull;
use SerdeValue::Number as SerdeNumber;
use SerdeValue::Object as SerdeObject;
use SerdeValue::String as SerdeString;
use crate::disc_objects::GatewayEventBinding;

type SerdeMap = serde_json::map::Map<String, SerdeValue>;

pub const DISCORD_API: &'static str = "https://discord.com/api";
pub const VALID_API: [u32; 3] = [7, 8, 9];
pub const LIBRARY_NAME: &'static str = "Celestial";

pub static USER_AGENT: &str = concat!(
"DiscordBot (",
"https://github.com/jean1398reborn/Celestial ",
env!("CARGO_PKG_VERSION"),
")",
);

#[derive(Debug)]
pub struct Client {
    pub token: String,
    pub api_url: String,
    pub request_client: reqwest::Client,
    pub gateway: Gateway,
    pub api_ver: u32,
    pub heartbeat_interval: u64,
    pub intents: Intent,
    pub sequence: Option<u64>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Payload {
    #[serde(rename = "op")]
    pub opcode: u32,
    #[serde(flatten)]
    pub data: Option<GatewayEvent>,
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
    pub gateway_type: disc_objects::GatewayEventBinding,
}

#[derive(Deserialize, Debug)]
pub struct Gateway {
    pub url: String,
    shards: u32,
    session_start_limit: HashMap<String, u64>,
}

#[derive(Debug)]
pub struct HttpRequest {
    extension: String,
    client: bot::BotClient,
}

bitflags::bitflags! {

    pub struct Intent: u32 {
        const GUILDS = 1 << 0;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_BANS = 1 << 2;
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag="t", content="d", rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum GatewayEvent {
    Hello(disc_objects::Hello),
    Ready(disc_objects::ReadyEvent),
    Resumed,
    Reconnect,
    InvalidSession,
    ChannelCreate(disc_objects::Channel),
    ChannelUpdate(disc_objects::Channel),
    ChannelDelete(disc_objects::Channel),
    ChannelPinsUpdate(disc_objects::ChannelPinUpdateEvent),
    ThreadCreate(disc_objects::Channel),
    ThreadUpdate(disc_objects::Channel),
    ThreadDelete(disc_objects::Channel),
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate(disc_objects::Message),
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd(disc_objects::ReactionAddEvent),
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
}

#[derive(thiserror::Error, Debug)]
pub enum DiscordError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

impl Client {
    pub fn new(mut api_ver: u32, token: String, intents: Vec<Intent>) -> Self {
        if !VALID_API.contains(&api_ver) {
            api_ver = 9
        }

        let api_url = format!("{}/v{}", DISCORD_API, api_ver);

        Self {
            token: token.clone(),
            api_url,
            request_client: Client::request_client_new(token),
            gateway: Gateway::new_placeholder(),
            api_ver,
            heartbeat_interval: 0,
            intents: intents.into_iter().collect(),
            sequence: None,
        }
    }

    pub async fn heartbeat(
        client: bot::BotClient,
        mut write_stream: &mut SplitSink<WbSS, Message>,
    ) {
        let heartbeat_interval = client.lock().await.heartbeat_interval.clone();
        let first_interval = heartbeat_interval as f64 * rand::random::<f64>();

        sleep(Duration::from_secs_f64(first_interval / 1000.0)).await;

        loop {
            let sequence = client.lock().await.sequence.clone();

            let sequence = match sequence.is_none() {
                true => SerdeNull,
                false => SerdeNumber(serde_json::Number::from(sequence.unwrap())),
            };

            Gateway::send(1, Some(sequence), None, None, &mut write_stream).await;

            sleep(Duration::from_millis(heartbeat_interval)).await;
        }
    }

    pub async fn keep_awake() {
        // could put connection checks, etc. in here
        loop {
            sleep(Duration::from_secs(30)).await;
        }
    }

    pub async fn identify(client : bot::BotClient, mut write_stream: &mut SplitSink<WbSS, Message>) {

        let client = client.clone();
        let client_guard = client.lock().await;

        let data = serde_json::json!({
            "token": client_guard.token,
            "intents": client_guard.intents.bits(),
            "properties": {
                "$os": std::env::consts::OS,
                "$browser": LIBRARY_NAME,
                "$device": LIBRARY_NAME,
            }
        });

        Gateway::send(2, Some(data), None, None, &mut write_stream).await;
    }


    pub async fn connect(client : bot::BotClient) -> (WbSS, tungstenite::handshake::client::Response) {
        let client = client.clone();

        let connection_url = url::Url::parse(client.lock().await.gateway.url.as_str()).unwrap();

        let (websocket_stream, response) = connect_async(connection_url)
            .await
            .expect("Connect_async failed to connect to gateway: ");

        (websocket_stream, response)
    }

    pub async fn check_hello(client : bot::BotClient, read: &mut SplitStream<WbSS>) {

        let client = client.clone();
        let hello_payload = Gateway::read_next_payload(read).await;

        let data = match hello_payload.data.unwrap() {
            GatewayEvent::Hello(hello_message) => hello_message,
            _ => panic!("Did not recieve hello gateway event")
        };

        let heartbeat_interval: u64 = data.heartbeat_interval;

        client.lock().await.heartbeat_interval = heartbeat_interval;
    }

    fn request_client_new(token: String) -> reqwest::Client {
        let builder = reqwest::ClientBuilder::new();
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_token = format!("Bot {}", token);
        let auth_header =
            reqwest::header::HeaderValue::from_str(&auth_token).expect("Invalid token");

        headers.insert("Authorization", auth_header);

        builder
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Request client build error")
    }
}
impl Gateway {
    pub fn new_placeholder() -> Self {
        Self {
            url: String::new(),
            shards: 1,
            session_start_limit: HashMap::new(),
        }
    }

    pub async fn run_events(client: bot::BotClient, gateway_event: GatewayEvent) {

        let client = client.clone();

    }

    pub async fn opcode_conversion(check_value : String) -> String {

        let check_value: serde_json::Value = serde_json::from_str(check_value.as_str()).expect("Failed to check opcode in json conversion");

        let opcode = check_value["op"].as_u64().expect(format!("{}", check_value["op"]).as_str());

        let returned_type = match opcode {
            11 => SerdeString(String::from("HEARTBEAT_OK")),
            10 => SerdeString(String::from("HELLO")),
            9 => SerdeString(String::from("INVALID_SESSION")),
            7 => SerdeString(String::from("RECONNECT")),
            1 => SerdeString(String::from("HEARTBEAT")),
            _ => check_value["t"].clone()
        };


        let return_string = serde_json::json!({
           "op": check_value["op"],
            "d": check_value["d"],
            "s": check_value["s"],
            "t": returned_type,
            "gateway_type": returned_type
        });

        serde_json::to_string(&return_string).unwrap()

    }

    pub async fn read_next_payload(read_stream: &mut SplitStream<WbSS>) -> Payload {

        let next_item = Gateway::opcode_conversion(
            read_stream
                .next()
                .await
                .expect("Gateway was closed when attempting to read next item")
                .expect("Error checking to see if connected")
                .to_string()).await;

        serde_json::from_str(next_item.as_str()).expect(format!("Failed converting next item in string to payload {} ",next_item).as_str())

    }

    pub async fn send(opcode: u64, data: Option<SerdeValue>, sequence: Option<u64>, gateway_type: Option<disc_objects::GatewayEventBinding>, sink: &mut SplitSink<WbSS, Message>) {

        let send_payload = serde_json::json!({
            "op": opcode,
            "d" : data,
            "s" : sequence,
            "t" : gateway_type,
        });

        let send_payload =
            serde_json::to_string(&send_payload).expect("Failed converting payload to string for sending");
        sink.send(Message::Text(send_payload)).await;
    }
}

impl HttpRequest {
    pub async fn str_new(extension: &str, client: bot::BotClient) -> Self {
        Self {
            extension: String::from(extension),
            client,
        }
    }

    pub async fn string_new(extension: String, client: bot::BotClient) -> Self {
        Self { extension, client }
    }

    pub async fn get(&self) -> Result<reqwest::Response, DiscordError> {
        let request_url = format!("{}{}", self.client.lock().await.api_url, self.extension);

        Ok(self
            .client
            .lock()
            .await
            .request_client
            .get(request_url)
            .send()
            .await?)
    }

    pub async fn post(&self, content: SerdeValue) -> Result<reqwest::Response, DiscordError> {
        let request_url = format!("{}{}", self.client.lock().await.api_url, self.extension);

        Ok(self
            .client
            .lock()
            .await
            .request_client
            .post(request_url)
            .json::<SerdeValue>(&content)
            .send()
            .await?)
    }
}

