use crate::settings;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::async_trait;
use super::super::util::message::MessageBasedCommand;
use super::super::util::Common;

pub struct Run {}

#[async_trait]
impl MessageBasedCommand for Run {
    fn name(&self) -> &str {
        "run"
    }
    
    fn able_to_register(&self) -> bool {
        !settings::DISABLE_RUN_FRAMEWORK
    }

    async fn run(&self, ctx: &Context, msg: &Message, _common: &Common) {
        // Implement your command logic here
        // For now, just a placeholder
        let _ = msg.reply(&ctx.http, "## still on maintenance").await;
        return;
    }
}

