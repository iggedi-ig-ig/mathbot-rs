pub mod latex;

use log::{debug, error, info, warn, LevelFilter};
use serenity::async_trait;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::borrow::Cow;

const TOKEN: &str = include_str!("../token.txt");

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        let content = &message.content;

        // message is latex (contains '$' at least twice)
        if content.find(|a| a == '$') != content.rfind(|a| a == '$') {
            info!("Got latex");

            let latex = latex::TEMPLATE
                .replace(r"<textcolor>", r"DBDBDB")
                .replace(r"<bgcolor>", r"36393E")
                .replace(r"<content>", content);
            let formula_result = latex::generate_png(&latex, message.id).await;

            match formula_result {
                Ok(image) => {
                    message
                        .channel_id
                        .send_files(
                            &ctx,
                            Some(AttachmentType::Bytes {
                                data: Cow::Owned(image),
                                filename: String::from("formula.png"),
                            }),
                            |m| m.content(""),
                        )
                        .await
                        .expect("failed to send message");
                }
                Err(why) => {
                    error!("error: {why:?}");

                    message
                        .channel_id
                        .send_files(
                            &ctx,
                            Some(AttachmentType::Bytes {
                                data: Cow::Owned(format!("{why:?}").as_bytes().to_vec()),
                                filename: String::from("log.txt"),
                            }),
                            |m| m.content("error log"),
                        )
                        .await
                        .expect("failed to reply to message after error");
                    return;
                }
            }
        }
    }

    async fn message_update(&self, _ctx: Context, _message_update_event: MessageUpdateEvent) {
        // info!("got message update: {:?}", message_update_event);

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

    // debug!("Token: {TOKEN}");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
