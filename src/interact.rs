use std::{env, sync::Arc};

use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
    Http, InstallationContext, InteractionContext,
};
use tracing::{debug, warn};

use crate::image_provider::nekos_best;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn register() -> CreateCommand {
    CreateCommand::new("interact")
        .description("harrass your friends")
        .integration_types(vec![InstallationContext::User, InstallationContext::Guild])
        .contexts(vec![
            InteractionContext::PrivateChannel,
            InteractionContext::Guild,
            InteractionContext::BotDm,
        ])
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "action", "action to perform")
                .add_string_choice("slap", "slap")
                .add_string_choice("hug", "hug")
                .add_string_choice("dance", "dance")
                .add_string_choice("nod", "nod")
                .add_string_choice("pat", "pat")
                .add_string_choice("kick", "kick"),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "user to harrass")
                .required(false),
        )
}

pub async fn run(cache_http: Arc<Http>, command: &CommandInteraction) -> CommandResult {
    // command
    //     .create_response(&cache_http, CreateInteractionResponse::Acknowledge)
    //     .await?;

    let options = command.data.options();
    let action = command.data.options.get(0).unwrap().value.as_str().unwrap();
    debug!("{:?}", action);

    let url = nekos_best(action).await;

    if url.is_err() {
        let err = url.unwrap_err();
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
                CreateInteractionResponseMessage::new().embed(CreateEmbed::new().title("title")),
            ),
        )
        .await?;
    Ok(())
}
