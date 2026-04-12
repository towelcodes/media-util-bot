use serenity::{
    all::{CommandInteraction, CreateAttachment, CreateInteractionResponseFollowup, ResolvedValue},
    prelude::*,
};
use tracing::debug;

use crate::{
    commands::CommandResult,
    process,
    util::{download_attachment, mark_processing},
};

pub async fn run(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
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
                        process::mask(file, include_bytes!("../assets/speech_bubble.png").into())?;
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
