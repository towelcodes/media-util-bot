use std::{env, time::Duration};

use serenity::{
    all::{
        ChannelId, CommandInteraction, CreateCommand, CreateCommandOption, CreateEmbed,
        CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateMessage, CreateModal, CreateQuickModal, InstallationContext, InteractionContext,
    },
    prelude::*,
};

use crate::commands::CommandResult;

pub fn register() -> CreateCommand {
    CreateCommand::new("request_feature")
        .description("anonymously ask the bot owner for a new feature (will it ever happen)")
        .integration_types(vec![InstallationContext::User, InstallationContext::Guild])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::PrivateChannel,
            InteractionContext::BotDm,
        ])
}

pub async fn run(
    ctx: &Context,
    cache_http: impl CacheHttp,
    command: &CommandInteraction,
) -> CommandResult {
    let modal = CreateQuickModal::new("Request a new feature")
        .timeout(Duration::from_secs(600))
        .short_field("Feature title")
        .paragraph_field("Long description (how should it work)");
    let response = command.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (title, description) = (&inputs[0], &inputs[1]);

    // send message to owner
    let owner = env::var("OWNER").unwrap_or("0".to_owned());
    let channel_id = env::var("UPDATE_CHANNEL").unwrap().parse::<u64>().unwrap();
    let channel = cache_http.http().get_channel(channel_id.into()).await?;
    channel
        .id()
        .send_message(
            &cache_http.http(),
            CreateMessage::new().content(format!(
                "<@{}> New feature request:\n**{}**\n\n{}",
                owner, title, description
            )),
        )
        .await?;

    response
        .interaction
        .create_response(
            &cache_http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .embed(
                        CreateEmbed::new()
                            .description("your request was submitted.")
                            .colour(0x57f287),
                    ),
            ),
        )
        .await?;
    Ok(())
}
