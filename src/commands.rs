use crate::process;
use crate::util::{download_attachment, mark_processing};
use serenity::all::{
    CacheHttp, CommandInteraction, CreateAttachment, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseFollowup, Mentionable, ResolvedOption,
    ResolvedValue,
};
use serenity::builder::CreateInteractionResponseMessage;
use serenity::http::Http;
use std::sync::Arc;
use tracing::debug;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub trait CommandExecutor {
    async fn run(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult;
}

pub async fn crush(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
    let file = download_attachment(command.data.options().get(0)).await?;
    mark_processing(&cache_http, &command).await;
    let arg = command
        .data
        .options
        .get(1)
        .map_or_else(|| None, |o| Some(o.value.as_i64().unwrap()));
    let (image, process) = process::crush(file, arg.unwrap_or(8) as u8)?;
    let builder = CreateInteractionResponseFollowup::new()
        .add_file(CreateAttachment::bytes(image, "output.png"))
        .content(format!(
            "-# applied `crush` ({}) | sent by {}",
            process,
            command.user.mention()
        ));
    command.create_followup(cache_http.http(), builder).await?;
    let _ = command.delete_response(cache_http.http()).await;
    Ok(())
}

pub async fn compress(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
    let file = download_attachment(command.data.options().get(0)).await?;
    mark_processing(&cache_http, &command).await;
    let arg = command
        .data
        .options
        .get(1)
        .map_or_else(|| None, |o| Some(o.value.as_i64().unwrap()));
    let (image, process) = process::compress(file, arg.unwrap_or(8) as u8)?;
    let builder = CreateInteractionResponseFollowup::new()
        .add_file(CreateAttachment::bytes(image, "output.jpg"))
        .content(format!(
            "-# applied `compress` ({}) | sent by {}",
            process,
            command.user.mention()
        ));
    command.create_followup(&cache_http, builder).await?;
    let _ = command.delete_response(cache_http.http()).await;
    Ok(())
}

pub async fn mask(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
    if let Some(opt) = command.data.options().get(0) {
        if let ResolvedValue::SubCommand(sub) = opt.value.clone() {
            debug!("{:?}", sub);
            match opt.name {
                "custom" => {
                    let file = download_attachment(sub.get(0)).await?;
                    let mask = download_attachment(sub.get(1)).await?;
                    mark_processing(&cache_http, &command).await;
                    let (image, process) = process::mask(file, mask)?;
                    let builder = CreateInteractionResponseFollowup::new()
                        .add_file(CreateAttachment::bytes(image, "output.png"))
                        .content(format!(
                            "-# applied `mask` ({}) | sent by {}",
                            process,
                            command.user.mention()
                        ));
                    command.create_followup(&cache_http.http(), builder).await?;
                    let _ = command.delete_response(&cache_http.http()).await;
                    ()
                }
                "speech_bubble" => {
                    let file = download_attachment(sub.get(0)).await?;
                    mark_processing(&cache_http, &command).await;
                    let (image, _) =
                        process::mask(file, include_bytes!("assets/speech_bubble.png").into())?;
                    let builder = CreateInteractionResponseFollowup::new()
                        .add_file(CreateAttachment::bytes(image, "output.png"))
                        .content(format!(
                            "-# applied `mask` (speech bubble) | sent by {}",
                            command.user.mention()
                        ));
                    command.create_followup(&cache_http.http(), builder).await?;
                    let _ = command.delete_response(&cache_http.http()).await;
                }
                _ => (), // not implemented
            }
        }
    }
    Ok(())
}

pub async fn ping(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
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

pub async fn cake(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
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
