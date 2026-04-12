use serenity::{
    all::{
        CommandInteraction, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    prelude::*,
};

use crate::commands::CommandResult;

pub async fn run(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
    let data = CreateInteractionResponseMessage::new().embed(
        CreateEmbed::new()
            .title("hey loser")
            .description("i'm still alive unfortunately")
            .colour(0xa6d189)
            .footer(CreateEmbedFooter::new(format!(
                "{}ms",
                chrono::Utc::now().timestamp_millis()
                    - command.id.created_at().unix_timestamp() * 1000
            ))),
    );
    let builder = CreateInteractionResponse::Message(data);
    command.create_response(cache_http.http(), builder).await?;
    Ok(())
}
