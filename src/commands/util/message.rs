use serenity::{model::channel::Message, prelude::Context, async_trait};
use super::Common;

#[async_trait]
pub trait MessageBasedCommand: Sync + Send {
    fn name(&self) -> &str;
    async fn run(&self, ctx: &Context, msg: &Message, common: &Common);
    fn able_to_register(&self) -> bool;
}
