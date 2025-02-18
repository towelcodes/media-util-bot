mod util;

use std::env;
use tracing::{debug, error, info, trace};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use serenity::prelude::*;
use serenity::{async_trait, Client};
use serenity::all::{CommandOptionType, CreateCommand, InstallationContext, Interaction, InteractionContext};
use serenity::builder::{CreateAttachment, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage};
use serenity::model::application::{Command, ResolvedOption, ResolvedValue};
use serenity::model::gateway::Ready;
use tracing_subscriber::layer::SubscriberExt;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = vec![
            CreateCommand::new("ping").description("ping pong")
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![InteractionContext::PrivateChannel, InteractionContext::Guild, InteractionContext::BotDm]),
            CreateCommand::new("cake").description("i will bake you a cake !!!!")
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![InteractionContext::PrivateChannel, InteractionContext::Guild, InteractionContext::BotDm]),
            CreateCommand::new("crush").description("crushes the bit depth of an uploaded image or audio")
                .add_option(CreateCommandOption::new(CommandOptionType::Attachment, "file", "the file you want to crush").required(true))
                .add_option(CreateCommandOption::new(CommandOptionType::Integer, "depth", "the target bit depth (1-8)").required(false))
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![InteractionContext::PrivateChannel, InteractionContext::Guild, InteractionContext::BotDm]),
            CreateCommand::new("compress").description("applies jpeg compression at your specified level")
                .add_option(CreateCommandOption::new(CommandOptionType::Attachment, "file", "the file you want to compress").required(true))
                .add_option(CreateCommandOption::new(CommandOptionType::Integer, "quality", "the quality of the compression (100 best, 0 worst)").required(false))
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![InteractionContext::PrivateChannel, InteractionContext::Guild, InteractionContext::BotDm]),
        ];
        let _ = Command::set_global_commands(&ctx.http, commands).await;

        debug!("commands registered");
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("received command: {command:#?}");

            let name = command.data.name.as_str();
            if name == "crush" || name == "compress" {
                // let file = command.data.options.get(0).unwrap().value.as_ref().unwrap().as_str().unwrap();
                if let Some(ResolvedOption {
                    value: ResolvedValue::Attachment(attachment), ..
                }) = command.data.options().get(0) {
                    debug!("performing operation on {attachment:#?}");
                    let data = CreateInteractionResponseMessage::new().content("image processing...");
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        error!("error responding: {why}");
                        return;
                    }

                    debug!("filetype is {:?}", attachment.content_type);
                    let file = attachment.download().await;
                    if let Err(why) = file {
                        error!("error downloading file: {why}");
                        let builder = CreateInteractionResponseFollowup::new().content("failed to obtain file");
                        let _ = command.create_followup(&ctx.http, builder).await;
                        return;
                    }

                    let file = file.unwrap();
                    let processed = match name {
                        "crush" => util::crush(file, command.data.options.get(1).map_or_else(|| 2, |o| 
                            num::clamp(o.value.as_i64().unwrap(), 1, 8) as u8)),
                        _ => util::compress(file, command.data.options.get(1).map_or_else(|| 50, |o| 
                            num::clamp(o.value.as_i64().unwrap(), 1, 100) as u8)),
                    };
                    if let Err(why) = processed {
                        error!("error processing file: {why}");
                        let builder = CreateInteractionResponseFollowup::new().content("failed to process file");
                        let _ = command.create_followup(&ctx.http, builder).await;
                        return;
                    }

                    let builder = CreateInteractionResponseFollowup::new()
                        .add_file(CreateAttachment::bytes(processed.unwrap(), format!("output.{}", match name { "crush" => "png", _ => "jpg" })))
                        .content("here's your stupid image back");
                    if let Err(why) = command.create_followup(&ctx.http, builder).await {
                        error!("error responding: {why}");
                    }
                }
                return;
            }

            let data = CreateInteractionResponseMessage::new().content("hey loser");
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                error!("error responding: {why}");
            }
        }
    }
}

#[dotenvy::load(path = "./.env", required = false)]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::builder().parse(env::var("RUST_LOG").unwrap_or("stupid_media_bot=info".into())).unwrap())
        .init();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("client error: {why:?}");
    }
}
