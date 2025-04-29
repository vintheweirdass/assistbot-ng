use serenity::all::{async_trait, CommandInteraction, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateInteractionResponseMessage, Interaction};
use reqwest;
use cmd_args_ext::{CommandArgsExt, CommandError}; // Assuming CommandArgsExt is in this module

use super::common::EmbedFromSettings;

pub type CirmResult = Result<CreateInteractionResponseMessage, CommandError>;
// Trait for slash commands with CommandArgs
#[async_trait]
pub trait SlashCommand: Sync + Send + 'static {
    fn register(&self) -> CreateCommand;
    fn able_to_register(&self) -> bool {
        return true;
    }
    // #[allow(unused_variables)]
    // fn before_run(&self, ctx: &Context, interaction: &Interaction, common: &before_run::ScCommon) -> Result<(), CommandError>  {
    //     return Ok(());
    // }
    async fn run(&self, ctx: &Context, interaction: &Interaction, common: &ScCommon) -> CirmResult;
}

pub struct ScCommon {
    pub command: CommandInteraction,
    pub http_client: reqwest::Client
}

impl ScCommon {
    pub fn reply(&self, value: impl Into<String>) -> CreateInteractionResponseMessage {
        return CreateInteractionResponseMessage::new().embed(CreateEmbed::new_from_settings().description(value));
    }
    pub fn reply_file(&self, value: CreateAttachment) -> CreateInteractionResponseMessage {
        return CreateInteractionResponseMessage::new().add_file(value);
    }
    pub fn reply_files(&self, value: impl IntoIterator<Item = CreateAttachment>) -> CreateInteractionResponseMessage {
        return CreateInteractionResponseMessage::new().add_files(value);
    }
    pub fn reply_embed(&self, value: CreateEmbed) -> CreateInteractionResponseMessage {
        return CreateInteractionResponseMessage::new().embed(value);
    }
    pub fn error(&self, message: impl Into<String>) -> CommandError {
        return CommandError::Default(message.into());
    }
    pub fn error_arg(&self, field_name:impl Into<String>, message: impl Into<String>) -> CommandError {
        return CommandError::Argument(field_name.into(), message.into());
    }
    pub fn parse_option<T: CommandArgsExt>(&self) -> Result<T, CommandError>{
        T::from_command(&self.command)
    }
}