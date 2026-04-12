use serenity::{
    all::{CommandInteraction, CreateAttachment, CreateInteractionResponseFollowup},
    prelude::*,
};

use crate::{
    commands::CommandResult,
    process,
    util::{download_attachment, mark_processing},
};

pub async fn run(cache_http: impl CacheHttp, command: &CommandInteraction) -> CommandResult {
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
