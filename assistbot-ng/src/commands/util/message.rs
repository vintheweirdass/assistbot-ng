use cmd_args_ext::CommandError;
use reqwest::Client;
use serenity::{all::{CreateAttachment, CreateEmbed, CreateInteractionResponseMessage, CreateMessage}, async_trait, model::channel::Message, prelude::Context};
use super::{common::EmbedFromSettings, slash::CirmResult};

#[async_trait]
pub trait MessageBasedCommand: Sync + Send {
    fn name(&self) -> &str;
    async fn run(&self, ctx: &Context, msg: &Message, common: &MbcCommon) -> CmResult;
    fn able_to_register(&self) -> bool {
        return true;
    }
}

pub struct MbcCommon {
    pub http_client:Client
}
pub type CmResult = Result<CreateMessage, CommandError>;

impl MbcCommon {
    pub fn reply(&self, value: impl Into<String>) -> CreateMessage {
        return CreateMessage::new().embed(CreateEmbed::new_from_settings().description(value));
    }
    pub fn reply_file(&self, value: CreateAttachment) -> CreateMessage {
        return CreateMessage::new().add_file(value);
    }
    pub fn reply_files(&self, value: impl IntoIterator<Item = CreateAttachment>) -> CreateMessage {
        return CreateMessage::new().add_files(value);
    }
    pub fn reply_embed(&self, value: CreateEmbed) -> CreateMessage {
        return CreateMessage::new().embed(value)
    }
    pub fn error(&self, message: impl Into<String>) -> CommandError {
        return CommandError::Default(message.into());
    }
    pub fn error_arg(&self, field_name:impl Into<String>, message: impl Into<String>) -> CommandError {
        return CommandError::Argument(field_name.into(), message.into());
    }
}