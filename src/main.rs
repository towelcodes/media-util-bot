mod process;
mod commands;
mod util;

use std::env;
use std::sync::Arc;
use tracing::{debug, error, info, trace};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use serenity::prelude::*;
use serenity::{async_trait, Client};
use serenity::all::{CommandOptionType, CreateCommand, InstallationContext, Interaction, InteractionContext};
use serenity::builder::{CreateAttachment, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse};
use serenity::model::application::{Command, ResolvedOption, ResolvedValue};
use serenity::model::gateway::Ready;
use serenity::model::Timestamp;
use tracing_subscriber::layer::SubscriberExt;
use include_dir::{include_dir, Dir};
use crate::process::mask;

static ASSETS: Dir = include_dir!("src/assets");

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
            CreateCommand::new("mask").description("hides some parts of an image based on a luma mask")
                .add_option(CreateCommandOption::new(CommandOptionType::Attachment, "image", "the image you want to mask").required(true))
                // .add_option(CreateCommandOption::new(CommandOptionType::Attachment, "mask", "the mask image").required(true))
                .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "custom", "use a custom mask image")
                    .add_sub_option(CreateCommandOption::new(CommandOptionType::Attachment, "mask", "the mask image where black = transparent").required(true)))
                .add_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "builtin", "use a builtin mask image")
                    .add_sub_option(CreateCommandOption::new(CommandOptionType::SubCommandGroup, "speech_bubble", "use a speech bubble mask")))
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![InteractionContext::PrivateChannel, InteractionContext::Guild, InteractionContext::BotDm]),
        ];
        let _ = Command::set_global_commands(&ctx.http, commands).await;

        debug!("commands registered");
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let name = command.data.name.as_str();
            let command_result = match name {
                "crush" => commands::crush(Arc::clone(&ctx.http), &command).await,
                "compress" => commands::compress(Arc::clone(&ctx.http), &command).await,
                "mask" => {
                    commands::mask(Arc::clone(&ctx.http), &command).await
                },
                "ping" => commands::ping(Arc::clone(&ctx.http), &command).await,
                _ => Ok(()),
            };

            if let Err(why) = command_result {
                error!("error running command: {why}");
                let embed = CreateEmbed::new().title("Something went wrong").description(format!("{why}")).colour(0xe78284);
                let builder = CreateInteractionResponseFollowup::new().embed(embed).ephemeral(true);
                let _ = command.create_followup(&ctx.http, builder).await;
                let _ = command.delete_response(&ctx.http).await;
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
