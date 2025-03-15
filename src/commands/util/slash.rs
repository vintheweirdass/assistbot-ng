use serenity::all::{async_trait, CommandInteraction, Context, CreateCommand, Interaction};
// use super::{Common};
// Trait for slash commands
#[async_trait]
pub trait SlashCommand: Sync + Send + 'static {
    fn register(&self) -> CreateCommand;
    fn able_to_register(&self) -> bool;
    async fn run(&self, ctx: &Context, interaction: &Interaction, common: &ScCommon) -> Result<Option<String>, String>;
}
pub struct ScCommon {
    pub command:CommandInteraction
}