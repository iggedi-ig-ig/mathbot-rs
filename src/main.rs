use log::{error, LevelFilter};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::event::MessageUpdateEvent;
use serenity::prelude::*;

const TOKEN: &str = include_str!("../token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message_update(
        &self,
        _ctx: Context,
        _old_if_available: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if let Some(message) = new {
            println!("got message: {message:?}");
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_module("mathbot-rs", LevelFilter::Debug)
        .init();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
