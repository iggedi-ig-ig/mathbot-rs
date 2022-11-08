pub mod latex;

use log::{debug, error, info, LevelFilter};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

const TOKEN: &str = include_str!("../token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        info!("got message: {:?}", message);

        // message is latex
        if message.content.contains('$') {
            let content = latex::generator::generate_png_api(
                &latex::TEMPLATE.replace("#CONTENT", r#"$\forall x \in \mathbb{R}$"#),
            )
            .await
            .unwrap();
        }
    }

    async fn message_update(&self, _ctx: Context, message_update_event: MessageUpdateEvent) {
        info!("got message update: {:?}", message_update_event);

        // TODO: support message updates
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("bot is ready! {}", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_module("mathbot_rs", LevelFilter::Debug)
        .init();

    debug!("Token: {TOKEN}");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
