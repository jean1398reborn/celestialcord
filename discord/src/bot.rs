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

pub type PinnedFuture = Box<dyn Fn(discord::GatewayEvent, BotClient) -> BoxFuture<'static, ()> + Send + Sync>;
pub type EventMap = Arc<RwLock<HashMap<disc_objects::GatewayEventBinding, PinnedFuture>>>;

pub type BotClient = Arc<Mutex<Client>>;
use crate::discord;

use serde_json::value::Value as SerdeValue;
use SerdeValue::Null as SerdeNull;
use SerdeValue::Number as SerdeNumber;
use SerdeValue::Object as SerdeObject;
use SerdeValue::String as SerdeString;
use crate::disc_objects::Snowflake;

type SerdeMap = serde_json::map::Map<String, SerdeValue>;

pub struct Bot
{
    pub client: BotClient,
    pub gateway_event_map: EventMap
}

impl Bot
{
    pub fn new(mut api_ver: u32, token: String, intents: Vec<Intent>) -> Self {

        let client = Client::new(api_ver, token, intents);
        let gateway_map = Arc::new(RwLock::new((HashMap::new())));

        Self {
            client: Arc::new(Mutex::new(client)),
            gateway_event_map: gateway_map,
        }
    }

    pub async fn add_event(&self, gateway_event: disc_objects::GatewayEventBinding, function: PinnedFuture) {
        let mut gateway_map = self.gateway_event_map.clone();

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

    pub async fn read(client: BotClient, read: &mut SplitStream<WbSS>, gateway_event_map: EventMap) {
        loop {
            let payload = Gateway::read_next_payload(read).await;

            if payload.sequence.is_some() {
                client.lock().await.sequence = payload.sequence;
            }

            if !payload.data.is_none() {

                let mut exists = gateway_event_map.clone().read().await.get(&payload.gateway_type).is_some();
                let map = gateway_event_map.clone();
                let client = client.clone();
                if exists {
                    tokio::spawn(async move {
                        map.clone().read().await.get(&payload.gateway_type).unwrap()(payload.data.unwrap(), client.clone()).await;
                    });
                }

            }

        }
    }

    pub async fn elevate(&self) {
        self.update_client_gateway().await;

        let (webstream, _) = discord::Client::connect(self.client.clone()).await;
        let (mut write, mut read) = webstream.split();

        discord::Client::identify(self.client.clone(), &mut write).await;
        discord::Client::check_hello(self.client.clone() ,&mut read).await;
        println!("Successfully connected to discord. ");

        let client = self.client.clone();
        tokio::spawn(async move {
            discord::Client::heartbeat(client.clone(), &mut write).await;
        });

        let client = self.client.clone();
        let gateway_event_map = self.gateway_event_map.clone();
        tokio::spawn(async move {
            Bot::read(client.clone(), &mut read, gateway_event_map.clone()).await;
        });

        Client::keep_awake().await;
    }

}
