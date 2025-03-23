use cmd_args::CommandArgs;
use cmd_args_ext::CreateCommandExt;
use serenity::all::{Context, CreateCommand, Interaction};
use serenity::async_trait;

use super::super::{util::slash::{ScCommon, SlashCommand, CirmResult}};

// CreateCommand::new("attachmentinput")
// .description("Test command for attachment input")
// .add_option(
//     CreateCommandOption::new(CommandOptionType::Attachment, "attachment", "A file")
//         .required(true),
// )
#[derive(Default, CommandArgs)]
pub struct Args {
    #[description("The name")]
    name: Option<String>
}
pub struct Hello {}
#[async_trait]
impl SlashCommand for Hello {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt_raw = &common.parse_option::<Args>();
        if opt_raw.is_err() {
            return common.reply("Hi!");
        }
        let opt = &opt_raw.as_ref().unwrap();
        let name = &opt.name;
        if name.is_none() {
            return common.reply("Hi!");
        }
        return common.reply(format!("Hi, {}!", name.as_ref().unwrap()));
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("hello")
        .add_args::<Args>()
        .description("You wasted 5 secs to see this")
    }
    
    fn able_to_register(&self) -> bool {
        true
    }
}