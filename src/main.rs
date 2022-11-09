pub mod latex;

use log::{debug, error, info, warn, LevelFilter};
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
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

        // message is latex
        if content.find('$') != content.rfind('$') {
            // message contains at least $ twice

            match latex::generator::generate_png_api(
                &latex::TEMPLATE.replace("#CONTENT", &format!(r"{:?}", content)),
                message.id,
            )
            .await
            {
                Ok(image) => {
                    message
                        .channel_id
                        .send_files(
                            &ctx,
                            Some(AttachmentType::Bytes {
                                data: Cow::Owned(image),
                                filename: "formula.png".to_string(),
                            }),
                            |m| m.content("formula"),
                        )
                        .await
                        .expect("failed to send message");
                }
                Err(why) => {
                    let mut msg_builder = MessageBuilder::new();

                    msg_builder.push_line("Malformed LaTeX detected:");
                    msg_builder.push_codeblock(&format!("{why:?}")[0..1900], None);

                    message
                        .reply_mention(&ctx, msg_builder)
                        .await
                        .expect("failed to reply to message");
                    return;
                }
            }

            let Ok(image) = latex::generator::generate_png_api(
                &latex::TEMPLATE.replace("#CONTENT", r#"$\forall x \in \mathbb{R}$"#),
                message.id,
            )
            .await else {
                return;
            };

            if let Err(why) = message.reply(&ctx, "LaTeX detected!").await {
                warn!("failed to reply to message: {why:?}");
            }
        }
    }

    async fn message_update(&self, _ctx: Context, message_update_event: MessageUpdateEvent) {
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
