use std::env;

use database::models::LlmMessage;
use openrouter_rs::{api::chat::ChatCompletionRequest, types::Role, Message};
use serenity::{
    all::{
        CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOptionType,
        CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
        CreateInteractionResponse, CreateInteractionResponseFollowup,
        CreateInteractionResponseMessage, EditInteractionResponse, InstallationContext,
        InteractionContext,
    },
    prelude::*,
};

use crate::{commands::CommandResult, AiClient, DatabasePool};

pub fn register() -> CreateCommand {
    CreateCommand::new("ai")
        .description("talk to me!!!")
        .integration_types(vec![InstallationContext::User, InstallationContext::Guild])
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::PrivateChannel,
            InteractionContext::BotDm,
        ])
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "ask", "talk to me!")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "prompt",
                        "message to send",
                    )
                    .required(true),
                ),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "forget_history",
            "i will forget your recent messages",
        ))
}

async fn ask(
    ctx: &Context,
    command: &CommandInteraction,
    subcommand_args: &Vec<CommandDataOption>,
) -> CommandResult {
    let data = CreateInteractionResponseMessage::new().content("> thinking...");
    let builder = CreateInteractionResponse::Message(data);
    command.create_response(&ctx.http(), builder).await?;

    // set model
    let model = env::var("OPENROUTER_MODEL").unwrap_or("x-ai/grok-4.1-fast".to_owned());
    let system_prompt =
        env::var("LLM_SYSTEM_PROMPT").unwrap_or("You are a helpful assistant.".to_owned());

    // retrive context
    let db_context = {
        let data_read = ctx.data.read().await;
        let pool = data_read
            .get::<DatabasePool>()
            .expect("Expected database pool to be available")
            .clone();
        let mut connection = pool
            .get()
            .expect("Expected database pool to have a available connection");
        database::get_context(&mut connection, command.user.id.into())
    };

    let mut history: Vec<LlmMessage> = if db_context.context.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&db_context.context).unwrap_or_else(|_| Vec::new())
    };

    let name = command.user.display_name();
    let prompt = subcommand_args.first().unwrap().value.as_str().unwrap();

    let mut messages = vec![Message::new(
        Role::System,
        format!(
            "{} The user sending the message has the display name '{name}'. You are running the LLM model '{}'.",
            system_prompt, model
        ),
    )];

    history.iter().for_each(|message| {
        messages.push(Message::new(
            match message.role.as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                _ => Role::System,
            },
            message.content.clone(),
        ));
    });

    messages.push(Message::new(Role::User, format!("{}", prompt)));

    let response = {
        let data_read = ctx.data.read().await;
        let client = data_read
            .get::<AiClient>()
            .expect("Expected AI client to be available")
            .read()
            .await;
        let request = ChatCompletionRequest::builder()
            .model(model)
            .messages(messages)
            .temperature(0.7)
            .max_tokens(500)
            .build()?;
        client.chat().create(&request).await?
    };

    let followup = EditInteractionResponse::new().content(format!(
        "{}\n-# (AI generated)",
        response.choices[0].content().unwrap_or("")
    ));
    command.edit_response(&ctx.http, followup).await?;

    // store history in the database
    history.push(LlmMessage {
        role: "user".to_string(),
        content: prompt.to_owned(),
    });
    history.push(LlmMessage {
        role: "assistant".to_string(),
        content: response.choices[0].content().unwrap_or("").to_string(),
    });

    {
        let data_read = ctx.data.read().await;
        let pool = data_read
            .get::<DatabasePool>()
            .expect("Expected database pool to be available")
            .clone();
        let mut connection = pool
            .get()
            .expect("Expected database pool to have a available connection");
        database::update_context(
            &mut connection,
            command.user.id.into(),
            serde_json::to_string(&history).unwrap(),
        );
    };

    Ok(())
}

async fn clear_history(ctx: &Context, command: &CommandInteraction) -> CommandResult {
    command
        .create_response(
            &ctx.http(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("nuh uh"),
            ),
        )
        .await?;
    Ok(())
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> CommandResult {
    if let Some(subcommand_opt) = command.data.options.first() {
        let subcommand = subcommand_opt.name.as_str();
        if let CommandDataOptionValue::SubCommand(subcommand_args) = &subcommand_opt.value {
            match subcommand {
                "ask" => {
                    return ask(ctx, command, subcommand_args).await;
                }
                "forget_history" => {
                    return clear_history(ctx, command).await;
                }
                _ => {}
            }
        }
    }

    // invalid command
    Ok(())
}
