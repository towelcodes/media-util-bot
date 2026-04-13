use serenity::all::{CacheHttp, CommandInteraction};

pub mod ai;
pub mod cake;
pub mod compress;
pub mod crush;
pub mod interact;
pub mod mask;
pub mod ping;
pub mod request;

pub type CommandResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
