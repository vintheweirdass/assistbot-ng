use cmd_args::CommandArgs;
use cmd_args_ext::{self, CreateCommandExt};
use serenity::all::{Context, CreateCommand, Interaction};
use serenity::async_trait;

use super::super::util::{slash::{CirmResult, ScCommon}};

use super::super::SlashCommand;
#[derive(Default, CommandArgs)]
struct Args {
    #[description("The prompt")]
    prompt: String,
}

pub struct Ask {}
#[async_trait]
impl <'a> SlashCommand for Ask {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt_raw = common.parse_option::<Args>();
        if let Err(err)=opt_raw {
            return Some(Err(err))
        }
        let prompt = &opt_raw.as_ref().unwrap().prompt;
        let raw = common.http_client.get(format!("https://text.pollinations.ai/{prompt}")).send().await;
            if let Err(err) = raw {
                return common.error(format!("Failed to fetch: {err}"))
            } 
            let res = raw.unwrap();
            let text_raw = res.text().await;
            if let Err(err) = text_raw {
                return common.error(format!("Failed to parse result: {err}"))
            }
            let text = text_raw.unwrap();
            if text.len()>=2000 {
                return common.error("The answer was too large. try asking another question")
            }
            return common.reply(text); 
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("ask")
        .add_args::<Args>()
        .description("ask something from text.pollination.ai")
    }
    
    fn able_to_register(&self) -> bool {
        true
    }
}