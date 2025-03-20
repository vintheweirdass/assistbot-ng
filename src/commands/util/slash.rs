use std::sync::Arc;

use cmd_args::CommandArgs;
use serenity::all::{async_trait, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateInteractionResponseMessage, Http, Interaction};
use reqwest;
use cmd_args_ext::{CommandArgsExt, CommandError}; // Assuming CommandArgsExt is in this module

pub type CirmResult = Option<Result<CreateInteractionResponseMessage, CommandError>>;
// Trait for slash commands with CommandArgs
#[async_trait]
pub trait SlashCommand: Sync + Send + 'static {
    fn register(&self) -> CreateCommand;
    fn able_to_register(&self) -> bool;
    async fn run(&self, ctx: &Context, interaction: &Interaction, common: &ScCommon) -> Option<Result<CreateInteractionResponseMessage, CommandError>>;
}

pub struct ScCommon {
    pub command: CommandInteraction,
    pub http_client: reqwest::Client
}

impl ScCommon {
    pub fn reply(&self, value: impl Into<String>) -> CirmResult {
        Some(Ok(CreateInteractionResponseMessage::new().content(value)))
    }
    pub fn reply_file(&self, value: CreateAttachment) -> CirmResult {
        Some(Ok(CreateInteractionResponseMessage::new().add_file(value)))
    }
    pub fn reply_files(&self, value: impl IntoIterator<Item = CreateAttachment>) -> CirmResult {
        Some(Ok(CreateInteractionResponseMessage::new().add_files(value)))
    }
    pub fn reply_embed(&self, value: CreateEmbed) -> CirmResult {
        Some(Ok(CreateInteractionResponseMessage::new().embed(value)))
    }
    pub fn reply_cirm(&self, value: CreateInteractionResponseMessage) -> CirmResult {
        Some(Ok(value))
    }
    pub fn error(&self, message: impl Into<String>) -> CirmResult {
        Some(Err(CommandError::Default(message.into())))
    }
    pub fn error_arg(&self, field_name:impl Into<String>, message: impl Into<String>) -> CirmResult {
        Some(Err(CommandError::Argument(field_name.into(), message.into())))
    }
    pub fn parse_option<T: CommandArgsExt>(&self) -> Result<T, CommandError>{
        T::from_command(&self.command)
    }
}