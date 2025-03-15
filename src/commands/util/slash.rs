use serenity::all::{async_trait, CommandInteraction, Context, CreateCommand, Interaction};
use reqwest::Client as HttpClient;

use super::common::CommandError;
// use super::{Common};
// Trait for slash commands
#[async_trait]
pub trait SlashCommand: Sync + Send + 'static {
    fn register(&self) -> CreateCommand;
    fn able_to_register(&self) -> bool;
    async fn run(&self, ctx: &Context, interaction: &Interaction, common: &ScCommon) -> Option<Result<String, CommandError>>;
}
pub struct ScCommon {
    pub command:CommandInteraction,
    pub http_client:HttpClient
}