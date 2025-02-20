use serenity::all::{CommandDataOption, CommandInteraction, CreateEmbed, ResolvedOption, ResolvedValue};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::http::CacheHttp;

pub async fn mark_processing(cache_http: impl CacheHttp, command: &CommandInteraction) {
    let builder = CreateInteractionResponseMessage::new().embed(
        CreateEmbed::new().title("Processing...").description("I'm processing your image...").colour(0xe5c890)).ephemeral(true);
    let data = CreateInteractionResponse::Message(builder);
    let _ = command.create_response(&cache_http, data).await;
}

pub async fn download_attachment(option: Option<&ResolvedOption<'_>>) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(ResolvedOption {
            value: ResolvedValue::Attachment(attachment), ..
        }) = option {
        let data = attachment.download().await?;
        Ok(data)
    } else {
        Err("No file provided".into())
    }
}