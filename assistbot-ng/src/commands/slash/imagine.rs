use cmd_args::CommandArgs;
use cmd_args_ext::{self, CreateCommandExt};
use serenity::all::{Context, CreateCommand, CreateEmbed, Interaction};
use serenity::async_trait;
use urlencoding::encode;

use crate::commands::util::common::EmbedFromSettings;

use super::super::util::{slash::{ScCommon, CirmResult}};

use super::super::SlashCommand;
#[derive(Default, CommandArgs)]
struct Args {
    #[description("The prompt")]
    prompt: String,
}

pub struct Imagine {}
#[async_trait]
impl <'a> SlashCommand for Imagine {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt = common.parse_option::<Args>()?;
        let prompt = format!("https://image.pollinations.ai/prompt/{}", encode(&opt.prompt.replace(" ", "+")).into_owned());
        return Ok(common.reply_embed(CreateEmbed::new_from_settings().image(prompt)));
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("imagine")
        .add_args::<Args>()
        .description("generate image from image.pollination.ai")
    }
}