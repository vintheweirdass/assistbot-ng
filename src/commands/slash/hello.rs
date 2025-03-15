use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use super::super::{util::{slash::{ScCommon, SlashCommand}, CommandError}};

// CreateCommand::new("attachmentinput")
// .description("Test command for attachment input")
// .add_option(
//     CreateCommandOption::new(CommandOptionType::Attachment, "attachment", "A file")
//         .required(true),
// )
pub struct Hello {}
#[async_trait]
impl SlashCommand for Hello {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> Option<Result<String, CommandError>> {
        let command = &common.command;
        if let Some(ResolvedOption {
            value: ResolvedValue::String(name), ..
        }) = command.data.options().first()
        {
            return Some(Ok(format!("Hi, {name}!").to_owned()));
        } else {
            return Some(Ok("Hi!".to_owned()));
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