use serenity::{
    all::{
        CommandInteraction, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    prelude::*,
};

use crate::commands::CommandResult;

pub async fn run(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
    let data = CreateInteractionResponseMessage::new().embed(
        CreateEmbed::new()
            .title("jk")
            .description("there's no cake for you")
            .colour(0xf9e2af),
    );
    let builder = CreateInteractionResponse::Message(data);
    command.create_response(cache_http.http(), builder).await?;
    Ok(())
}
