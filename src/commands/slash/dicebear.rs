use super::super::shared::dicebear::StyleVariations; 
use cmd_args::{CommandArgs};
use cmd_args_ext::{CommandError, CommandOptionTypeExt, CreateCommandExt, EnumArgsExt};
use serenity::all::{CommandOptionType, Context, CreateAttachment, CreateCommand, CreateEmbed, CreateInteractionResponseMessage, Interaction};
use serenity::async_trait;
use dicebear::generate as dicebear_generate;
use tracing::info;

use super::super::{util::slash::{ScCommon, SlashCommand, CirmResult}};

// CreateCommand::new("attachmentinput")
// .description("Test command for attachment input")
// .add_option(
//     CreateCommandOption::new(CommandOptionType::Attachment, "attachment", "A file")
//         .required(true),
// )

#[derive(Default, CommandArgs)]
struct Args {
    #[description("The name")]
    style: StyleVariations,
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
        info!("{}", style.to_alias());
        let gen = dicebear_generate(
            style.to_alias(),
            opt.seed.clone(), // OPTIONAL: Seed
            size, // REQUIRED: Size
            opt.flip, // OPTIONAL: Flip (bool)
            None
            ).await;
        if let Ok(img) = gen {
                let image_res = CreateAttachment::bytes(img.as_bytes(), "result.png");
                let embed = CreateEmbed::new()
                    .title("Local Image")
                    .image("attachment://result.png");

            return common.reply_cirm( CreateInteractionResponseMessage::new().add_embed(embed).add_file(image_res));
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