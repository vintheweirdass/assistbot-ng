use cmd_args::CommandArgs;
use cmd_args_ext::CreateCommandExt;
use serenity::all::{Context, CreateCommand, Interaction};
use serenity::async_trait;

use super::super::{util::slash::{ScCommon, SlashCommand, CirmResult}};

#[derive(Default, CommandArgs)]
pub struct Args {
    #[description("The name")]
    name: Option<String>
}
pub struct Thanks {}
#[async_trait]
impl SlashCommand for Thanks {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt_raw = &common.parse_option::<Args>();
        if opt_raw.is_err() {
            return Ok(common.reply("Thank you!"));
        }
        let opt = &opt_raw.as_ref().unwrap();
        let name = &opt.name;
        if name.is_none() {
            return Ok(common.reply("Thank you!"));
        }
        return Ok(common.reply(format!("Thanks, {}!", name.as_ref().unwrap())));
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("thanks")
        .add_args::<Args>()
        .description("thanking to anyone is allowed in this bot.. i think")
    }
}