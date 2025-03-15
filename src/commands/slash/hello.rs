use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use super::super::{SlashCommand, util::slash::ScCommon};

// CreateCommand::new("attachmentinput")
// .description("Test command for attachment input")
// .add_option(
//     CreateCommandOption::new(CommandOptionType::Attachment, "attachment", "A file")
//         .required(true),
// )
pub struct Hello {}
#[async_trait]
impl SlashCommand for Hello {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> Result<Option<String>, String> {
        let command = &common.command;
        if let Some(ResolvedOption {
            value: ResolvedValue::String(name), ..
        }) = command.data.options().first()
        {
            return Ok(Some(format!("Hi {name}").to_owned()));
        } else {
            return Ok(Some("Hi".to_owned()));
        }
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("hello")
        .description("You wasted 5 secs to see this")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "name", "The name"),
        )
    }
    
    fn able_to_register(&self) -> bool {
        true
    }
}