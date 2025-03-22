use super::super::shared::dicebear::StyleVariations; 
use super::super::util::common::EmbedFromSettings;
use cmd_args::{CommandArgs};
use cmd_args_ext::{CreateCommandExt, EnumArgsExt};
use serenity::all::{Context, CreateCommand, CreateEmbed, Interaction};
use serenity::async_trait;
// currenly experimenting on non crate, im not rlly interested well if
// theres too many concurrent commands that running this
use dicebear::{generate_url as dicebear_generate_url};

use super::super::{util::slash::{ScCommon, SlashCommand, CirmResult}};

#[derive(Default, CommandArgs)]
struct Args {
    #[description("The name")]
    style:StyleVariations,
    #[description("Seed")]
    seed:Option<String>,
    #[description("Flip the image?")]
    flip:Option<bool>,
    #[description("Image size (default to: 256)")]
    size:Option<u32>
}
pub struct Dicebear {}
#[async_trait]
impl SlashCommand for Dicebear {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt_raw = common.parse_option::<Args>();
        if let Err(err)=opt_raw {
            return Some(Err(err))
        }
        let opt = opt_raw.as_ref().unwrap();
        let style = &opt.style;
        let size = if let Some(v) = &opt.size {
           v.to_string()
        } else {
           String::from("256")
        };
        let gen = dicebear_generate_url(
            "png".to_string(),
            style.to_alias(),
            opt.seed.clone(), // OPTIONAL: Seed
            size, // REQUIRED: Size
            opt.flip, // OPTIONAL: Flip (bool)
            None
            ).await;
        if let Ok(img) = gen {
                let embed = CreateEmbed::new_from_settings()
                    .image(img);

            return common.reply_embed(embed);
        } else {
            return common.error("Failed to generate image")
        }
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("dicebear")
        .add_args::<Args>()
        .description("Generate avatars from dicebear")
    }
    
    fn able_to_register(&self) -> bool {
        true
    }
}