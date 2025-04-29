use cmd_args::CommandArgs;
use cmd_args_ext::CreateCommandExt;
use serenity::all::{Context, CreateCommand, CreateEmbed, Interaction};
use serenity::async_trait;


use super::super::super::commands::ENUMS;

use super::super::{util::{slash::{ScCommon, SlashCommand, CirmResult}, common::EmbedFromSettings}};

#[derive(Default, CommandArgs)]
pub struct Args {
    #[description("The name")]
    name: String
}
pub struct EnumList {}
#[async_trait]
impl SlashCommand for EnumList {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt = common.parse_option::<Args>()?;
        let selected = opt.name.to_lowercase();
        let enums = &*ENUMS;
        for (name, vs) in enums {
            let to_low = &name.to_lowercase();
            if *to_low != selected {
                continue
            }
            let values: Vec<String> = vs
                .iter()
                .map(|value| format!("`{}`", value))
                .collect();
        
            return Ok(common.reply_embed(CreateEmbed::new_from_settings()
                .title(format!("Enum variant of `{name}`"))
                .description(values.join(", "))));
        }
        return Err(common.error("Enum wasnt found"));
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("enum")
        .add_args::<Args>()
        .description("Get the dictionary of enum")
    }

}