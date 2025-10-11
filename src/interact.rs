use std::sync::Arc;

use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage, Http, InstallationContext, InteractionContext,
};
use tracing::warn;

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
                .required(true)
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
    let action = command.data.options.get(0).unwrap().value.as_str().unwrap();
    let target = {
        if let Some(t) = command.data.options.get(1) {
            let user = cache_http
                .get_user(t.value.as_user_id().unwrap())
                .await
                .unwrap();
            user.display_name().to_owned()
        } else {
            "".to_owned()
        }
    };

    let image = nekos_best(action).await;

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
                        .title(format!(
                            "{} {}s {}",
                            command.user.display_name(),
                            action,
                            target
                        ))
                        .image(image.unwrap().url)
                        .footer(CreateEmbedFooter::new("powered by nekos.best"))
                        .colour(0x7cfa78),
                ),
            ),
        )
        .await?;
    Ok(())
}
