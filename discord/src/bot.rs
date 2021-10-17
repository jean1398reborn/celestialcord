use crate::discord::{Client, Gateway, HttpRequest, Intent, Payload, LIBRARY_NAME, GatewayEvent};
use futures::lock::Mutex;
use tokio::sync::RwLock;
use futures::future::{FutureExt, BoxFuture, Future};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use std::ops::Deref;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
type WbSS = WebSocketStream<MaybeTlsStream<TcpStream>>;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use crate::disc_objects;
use std::pin::Pin;

use crate::interactions;
use interactions::PinnedInteraction;
use interactions::AppCommandMap;

pub type PinnedFuture = Box<dyn Fn(discord::GatewayEvent, BotClient) -> BoxFuture<'static, ()> + Send + Sync>;
pub type EventMap = Arc<RwLock<HashMap<disc_objects::GatewayEventBinding, PinnedFuture>>>;

pub type BotClient = Arc<Mutex<Client>>;
use crate::discord;

use serde_json::value::Value as SerdeValue;
use SerdeValue::Null as SerdeNull;
use SerdeValue::Number as SerdeNumber;
use SerdeValue::Object as SerdeObject;
use SerdeValue::String as SerdeString;
use crate::disc_objects::{GatewayEventBinding, Snowflake};
use crate::interactions::{AppCommand, InteractionCommand};

type SerdeMap = serde_json::map::Map<String, SerdeValue>;

pub struct Bot
{
    pub client: BotClient,
    pub gateway_event_map: EventMap,
    pub interaction_map: AppCommandMap,
    pub sync_commands: bool,

}

impl Bot
{
    pub fn new(mut api_ver: u32, token: String, intents: Vec<Intent>) -> Self {

        let client = Client::new(api_ver, token, intents);
        let gateway_event_map = Arc::new(RwLock::new(HashMap::new()));
        let interaction_map = Arc::new(RwLock::new(HashMap::new()));

        Self {
            client: Arc::new(Mutex::new(client)),
            gateway_event_map,
            interaction_map,
            sync_commands: false,
        }
    }

    pub async fn sync_commands(&mut self) {
        self.sync_commands = true;
    }

    pub async fn add_event(&self, gateway_event: disc_objects::GatewayEventBinding, function: PinnedFuture) {
        let gateway_map = self.gateway_event_map.clone();

        gateway_map.write().await.insert(gateway_event, function);
    }

    pub async fn update_client_gateway(&self) {
        let gateway_request = HttpRequest::str_new("/gateway/bot", self.client.clone());

        self.client.lock().await.gateway = gateway_request
            .await
            .get()
            .await
            .expect("Failed to get gateway update request")
            .json::<Gateway>()
            .await
            .expect("Failed to parse gateway update request into Gateway Struct");
    }

    pub async fn read(client: BotClient, read: &mut SplitStream<WbSS>, gateway_event_map: EventMap, interaction_map: Option<AppCommandMap>) -> Payload {
        let payload = Gateway::read_next_payload(read).await;
        let result = payload.clone();

        if payload.sequence.is_some() {
            client.lock().await.sequence = payload.sequence.clone();
        }

        if !payload.data.is_none() {

            if payload.gateway_type == GatewayEventBinding::InteractionCreate {

                let data = payload.data.clone();
                let client = client.clone();
                tokio::spawn(async move {
                    Bot::handle_interaction(data, interaction_map.clone(), client.clone()).await;
                });
            }

            let exists = gateway_event_map.clone().read().await.get(&payload.gateway_type).is_some();
            let map = gateway_event_map.clone();
            let client = client.clone();

            if exists {
                tokio::spawn(async move {
                    map.clone().read().await.get(&payload.gateway_type).unwrap()(payload.data.unwrap(), client.clone()).await;
                });
            }

        }

        result
    }

    pub async fn check_events(&self, read: &mut SplitStream<WbSS>) {

        let hello_payload = Bot::read(self.client.clone(),  read, self.gateway_event_map.clone(), None).await;

        let data = match hello_payload.data.unwrap() {
            GatewayEvent::Hello(hello_message) => hello_message,
            _ => panic!("Did not recieve hello gateway event")
        };

        let heartbeat_interval: u64 = data.heartbeat_interval;

        let ready_event = Bot::read(self.client.clone(), read, self.gateway_event_map.clone(), None).await;

        let data = match ready_event.data.unwrap() {
            GatewayEvent::Ready(ready_event) => ready_event,
            _ => panic!("Ready event was not recieved,")
        };

        let application = data.application.id;

        let mut client_lock = self.client.lock().await;

        client_lock.application_id = application;
        client_lock.heartbeat_interval = heartbeat_interval;

    }

    pub async fn elevate(&self) {
        self.update_client_gateway().await;

        let (webstream, _) = discord::Client::connect(self.client.clone()).await;
        let (mut write, mut read) = webstream.split();

        discord::Client::identify(self.client.clone(), &mut write).await;
        self.check_events(&mut read).await;

        let client = self.client.clone();

        if self.sync_commands == true {

            let interaction_map = self.interaction_map.clone();
            tokio::spawn(async move {
                Bot::register_commands(client, interaction_map.clone()).await;
            });
        }

        let client = self.client.clone();
        tokio::spawn(async move {
            discord::Client::heartbeat(client, &mut write).await;
        });

        let client = self.client.clone();
        let interaction_map = self.interaction_map.clone();
        let gateway_event_map = self.gateway_event_map.clone();
        tokio::spawn(async move {
            loop {
                Bot::read(client.clone(), &mut read, gateway_event_map.clone(), Some(interaction_map.clone())).await;
            }
        });

        Client::keep_awake().await;
    }

}
