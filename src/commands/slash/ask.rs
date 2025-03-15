use serde::{Deserialize, Serialize};
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use super::super::util::CommandError;

use super::super::{SlashCommand, util::slash::ScCommon};

pub struct Ask {}
#[async_trait]
impl SlashCommand for Ask {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> Option<Result<String, CommandError>> {
        let command = &common.command;
        if let Some(ResolvedOption {
            value: ResolvedValue::String(prompt), ..
        }) = command.data.options().first()
        {
            let raw = common.http_client.get(format!("https://text.pollinations.ai/{}", prompt)).send().await;
            if let Err(err) = raw {
                return Some(Err(CommandError::Default(format!("Failed to fetch: {err}").to_owned())))
            } 
            let res = raw.unwrap();
            let text_raw = res.text().await;
            if let Err(err) = text_raw {
                return Some(Err(CommandError::Default(format!("Failed to parse result: {err}").to_owned())))
            }
            let text = text_raw.unwrap();
            if text.len()>=2000 {
                return Some(Err(CommandError::Default("The answer was too large. try asking another question")))
            }
            return Some(Ok(text));
        } else {
            return Some(Err(CommandError::Argument("prompt".to_owned(), "Missing prompt".to_owned())));
        }
        
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("ask")
        .description("ask something from text.pollination.ai")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "prompt", "The prompt")
            .required(true)
        )
    }
    
    fn able_to_register(&self) -> bool {
        true
    }
}