mod commands;
mod image_provider;
mod process;
mod util;
mod yuri;

use image::imageops::FilterType::Lanczos3;
use include_dir::{include_dir, Dir};
use openrouter_rs::OpenRouterClient;
use serenity::all::{
    ClientBuilder, CommandDataOptionValue, CommandOptionType, CreateCommand, GuildId, Http,
    HttpBuilder, InstallationContext, Interaction, InteractionContext,
};
use serenity::builder::{CreateCommandOption, CreateEmbed, CreateInteractionResponseFollowup};
use serenity::model::application::Command;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use serenity::{async_trait, Client};
use std::env;
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

struct DatabasePool;
impl TypeMapKey for DatabasePool {
    type Value = database::DbPool;
}

struct AiClient;
impl TypeMapKey for AiClient {
    type Value = Arc<RwLock<OpenRouterClient>>;
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = vec![
            commands::interact::register(),
            commands::request::register(),
            commands::ai::register(),
            yuri::register_yuri(),
            yuri::register_yaoi(),
            CreateCommand::new("ping")
                .description("ping pong")
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![
                    InteractionContext::PrivateChannel,
                    InteractionContext::Guild,
                    InteractionContext::BotDm,
                ]),
            CreateCommand::new("cake")
                .description("i will bake you a cake !!!!")
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![
                    InteractionContext::PrivateChannel,
                    InteractionContext::Guild,
                    InteractionContext::BotDm,
                ]),
            CreateCommand::new("crush")
                .description("crushes the bit depth of an uploaded image or audio")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Attachment,
                        "file",
                        "the file you want to crush",
                    )
                    .required(true),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "depth",
                        "the target bit depth (1-8)",
                    )
                    .required(false),
                )
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![
                    InteractionContext::PrivateChannel,
                    InteractionContext::Guild,
                    InteractionContext::BotDm,
                ]),
            CreateCommand::new("compress")
                .description("applies jpeg compression at your specified level")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Attachment,
                        "file",
                        "the file you want to compress",
                    )
                    .required(true),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "quality",
                        "the quality of the compression (100 best, 0 worst)",
                    )
                    .required(false),
                )
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![
                    InteractionContext::PrivateChannel,
                    InteractionContext::Guild,
                    InteractionContext::BotDm,
                ]),
            CreateCommand::new("mask")
                .description("hides some parts of an image based on a luma mask")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "custom",
                        "use a custom mask image",
                    )
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::Attachment,
                            "image",
                            "the image you want to mask",
                        )
                        .required(true),
                    )
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::Attachment,
                            "mask",
                            "the mask image where black = transparent",
                        )
                        .required(true),
                    ),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "speech_bubble",
                        "use a speech bubble mask",
                    )
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::Attachment,
                            "image",
                            "the image you want to ask",
                        )
                        .required(true),
                    ),
                )
                .integration_types(vec![InstallationContext::User])
                .contexts(vec![
                    InteractionContext::PrivateChannel,
                    InteractionContext::Guild,
                    InteractionContext::BotDm,
                ]),
        ];
        // For faster testing, register commands to a specific guild
        if let Ok(guild_id_str) = env::var("GUILD_ID") {
            if let Ok(guild_id) = guild_id_str.parse::<u64>() {
                let guild_id = GuildId::new(guild_id);
                let _ = guild_id.set_commands(&ctx.http, commands.clone()).await;
                debug!("Guild commands registered for guild {}", guild_id);
            } else {
                error!("Invalid GUILD_ID format");
            }
        }
        if let Err(e) = Command::set_global_commands(&ctx.http, commands).await {
            warn!("Error setting global commands: {:?}", e);
        } else {
            debug!("Global commands registered");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let name = command.data.name.as_str();
            let command_result = match name {
                "ai" => commands::ai::run(&ctx, &command).await,
                "crush" => commands::crush::run(Arc::clone(&ctx.http), &command).await,
                "compress" => commands::compress::run(Arc::clone(&ctx.http), &command).await,
                "mask" => commands::mask::run(Arc::clone(&ctx.http), &command).await,
                "ping" => commands::ping::run(Arc::clone(&ctx.http), &command).await,
                "cake" => commands::cake::run(Arc::clone(&ctx.http), &command).await,
                "interact" => commands::interact::run(Arc::clone(&ctx.http), &command).await,
                "request_feature" => {
                    commands::request::run(&ctx, Arc::clone(&ctx.http), &command).await
                }
                "yuri" => {
                    yuri::run(
                        Arc::clone(&ctx.http),
                        &command,
                        "yuri rating:general sort:random",
                    )
                    .await
                }
                "yaoi" => {
                    yuri::run(
                        Arc::clone(&ctx.http),
                        &command,
                        "yaoi rating:general sort:random",
                    )
                    .await
                }
                _ => {
                    error!("Unknown command: {}", name);
                    Ok(())
                }
            };

            info!("command: {:?}", command.data);
            if let Err(why) = command_result {
                error!("error running command: {why}");
                let embed = CreateEmbed::new()
                    .title("Something went wrong")
                    .description(format!("{why}"))
                    .colour(0xe78284);
                let builder = CreateInteractionResponseFollowup::new()
                    .embed(embed)
                    .ephemeral(true);
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
        .with(
            EnvFilter::builder()
                .parse(env::var("RUST_LOG").unwrap_or("media_bot=info".into()))
                .unwrap(),
        )
        .init();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    // setup database
    let db = database::establish_connection_pool();

    // setup ai features
    let or_client = OpenRouterClient::builder()
        .api_key(env::var("OPENROUTER_KEY").expect("OPENROUTER_API_KEY must be set"))
        .x_title("discordbot")
        .build()
        .expect("openrouter client failed to build");

    let http = HttpBuilder::new(token).build();
    let mut client = ClientBuilder::new_with_http(http, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Client creation failed.");

    // insert data
    {
        let mut data = client.data.write().await;
        data.insert::<AiClient>(Arc::new(RwLock::new(or_client)));
        data.insert::<DatabasePool>(db);
    }

    if let Err(why) = client.start().await {
        error!("client error: {why:?}");
    }
}
