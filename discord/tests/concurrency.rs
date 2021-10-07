use futures::future::{FutureExt};

extern crate celestialcord;

use celestialcord::bot;
use celestialcord::discord;
use celestialcord::disc_objects;
use celestialcord::bot::{Bot};
use celestialcord::discord::Intent;
use celestial_macros::add_fn;
use tokio::time::{Instant};

async fn on_ready(returned: discord::GatewayEvent, client: bot::BotClient) {
    println!("Bot ready!");
}

async fn response(message : disc_objects::Message, client: bot::BotClient) {
    let embed = disc_objects::Embed::new("Hello", "Snootiermoon!", 0xFF0000)
        .image("https://c.tenor.com/zDUT9yR2Zz0AAAAC/big-buger-eat-buger.gif");

    let mut reply = disc_objects::ReplyMessage::new(false)
        .add_embed(embed)
        .reply_message(message.clone());

    let response = reply.send(message.channel_id.clone(), client.clone()).await;

}

async fn longtask(message : disc_objects::Message, client: bot::BotClient) {
    let mut reply = disc_objects::ReplyMessage::new(false)
        .content_str("Task started")
        .reply_message(message.clone())
        .send(message.channel_id.clone(), client.clone()).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let mut reply = disc_objects::ReplyMessage::new(false)
        .content_str("Task finished")
        .reply_message(message.clone())
        .send(message.channel_id.clone(), client.clone()).await;
}

async fn on_message(returned: discord::GatewayEvent, client: bot::BotClient) {

    //check if we recieved the correct gateway event
    let message = match returned {
        discord::GatewayEvent::MessageCreate(message) => message,
        _ => panic!("Did not recieve message at on_message")
    };

    if message.is_bot() {
        return
    }

    // long task
    if message.content == "!longtask" {
        longtask(message.clone(), client.clone()).await;
    }

    // simple response command to test during longtask
    if message.content == "!response" {
        response(message.clone(), client.clone()).await;
    }

}



#[tokio::test]
async fn main() {
    // make bot
    let mut bot = Bot::new(
        9,
        std::env::var("BOT_TOKEN").expect("Put bot token in env_var!"),
        vec![Intent::all()],
    );


    // add event system, where you add a function to be called to an event.
    bot.add_event(disc_objects::GatewayEventBinding::MessageCreate, add_fn!(on_message)).await;
    bot.add_event(disc_objects::GatewayEventBinding::Ready, add_fn!(on_ready)).await;

    //alive
    bot.elevate().await;
}
