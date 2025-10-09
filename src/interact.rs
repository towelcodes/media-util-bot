use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, InstallationContext, InteractionContext,
};

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
            CreateCommandOption::new(
                CommandOptionType::SubCommandGroup,
                "action",
                "action to perform",
            )
            .add_sub_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "hug",
                "hug someone",
            )),
        )
}
