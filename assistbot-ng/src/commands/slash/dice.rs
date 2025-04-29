use std::num::NonZeroU8;

use cmd_args::CommandArgs;
use cmd_args_ext::CreateCommandExt;
use serenity::{all::{Context, CreateCommand, Interaction}, async_trait};

use crate::commands::util::slash::{CirmResult, ScCommon, SlashCommand};

#[allow(unused_imports)]
use rand::{Rng, rng};

#[derive(CommandArgs)]
pub struct Args {
    #[description("Dice range")]
    range_to:Option<NonZeroU8>
}
pub struct Dice;

#[async_trait]
impl SlashCommand for Dice {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> CirmResult {
        let opt_raw = common.parse_option::<Args>().ok();
        // this number is non zero btw
        let mut range_to = NonZeroU8::new(6).unwrap();
        if let Some(opt) = opt_raw {
            if let Some(range) = opt.range_to {
                range_to = range;
            };
        }
        let mut th_rng = rng();
        let num:u8 = th_rng.random_range(1..range_to.into());
        return Ok(common.reply(num.to_string()))
    }
    fn register(&self) -> CreateCommand {
        return CreateCommand::new("dice")
            .add_args::<Args>()
            .description("Roll a die! (yes, die. since only one)")
    }
}