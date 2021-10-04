use futures::future::{FutureExt};

extern crate celestialcord;

use celestialcord::bot;
use celestialcord::discord;
use celestialcord::disc_objects;
use celestialcord::bot::{Bot};
use celestialcord::discord::Intent;
use utility_macros::add_fn;
use tokio::time::{Instant};

async fn on_ready(returned: discord::GatewayEvent, client: bot::BotClient) {
    println!("Bot ready!");
}

async fn on_message(returned: discord::GatewayEvent, client: bot::BotClient) {

    //check if we recieved the correct gateway event
    let message = match returned {
        discord::GatewayEvent::MessageCreate(message) => message,
        _ => panic!("Did not recieve message at on_message")
    };

    //bot checks to make sure that the author is not a bot
    let isbot = match message.author.bot {
        Some(value) => value,
        None => false,
    };

    if isbot {
        return
    }

    // long task
    if message.content == "!longtask" {
        Bot::channel_id_send_text(message.channel_id.clone(), client.clone(), "Task started", false).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        Bot::channel_id_send_text(message.channel_id.clone(), client.clone(), "Task finished", false).await;
    }

    // simple response command to test during longtask
    if message.content == "!response" {
        let message_reply = format!("Hello there <@{}>", message.author.id.clone());

        Bot::channel_id_send_text(message.channel_id.clone(), client.clone(), message_reply.as_str(), false).await;
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
