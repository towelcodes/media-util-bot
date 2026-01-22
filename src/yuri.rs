use std::sync::Arc;

use serenity::all::{
    CacheHttp, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage, Http, InstallationContext, InteractionContext,
};
use tracing::warn;

use crate::image_provider::safebooru;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn register_yuri() -> CreateCommand {
    CreateCommand::new("yuri")
        .description("guess")
        .integration_types(vec![InstallationContext::User, InstallationContext::Guild])
        .contexts(vec![
            InteractionContext::PrivateChannel,
            InteractionContext::Guild,
            InteractionContext::BotDm,
        ])
}

pub fn register_yaoi() -> CreateCommand {
    CreateCommand::new("yaoi")
        .description("guess")
        .integration_types(vec![InstallationContext::User, InstallationContext::Guild])
        .contexts(vec![
            InteractionContext::PrivateChannel,
            InteractionContext::Guild,
            InteractionContext::BotDm,
        ])
}

pub async fn run(
    cache_http: impl CacheHttp,
    command: &CommandInteraction,
    query: &str,
) -> CommandResult {
    let image = safebooru(query).await;

    if image.is_err() {
        let err = image.unwrap_err();
        warn!("{:?}", err);
        command
            .create_response(
                &cache_http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Error running command: {:?}", &err)),
                ),
            )
            .await?;
        return Ok(());
    }

    command
        .create_response(
            &cache_http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(
                    CreateEmbed::new()
                        .image(image.unwrap().url)
                        .footer(CreateEmbedFooter::new("powered by safebooru.org"))
                        .colour(0x7cfa78),
                ),
            ),
        )
        .await?;
    Ok(())
}
