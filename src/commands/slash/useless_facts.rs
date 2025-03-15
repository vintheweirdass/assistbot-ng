use serde::{Deserialize, Serialize};
use serenity::all::{Context, CreateCommand, Interaction, ResolvedOption, ResolvedValue};
use serenity::async_trait;
use super::super::util::CommandError;

use super::super::{SlashCommand, util::slash::ScCommon};
#[derive(Serialize, Deserialize)]
struct Fact {
    id:String,
    text:String,
    source:String,
    source_url:String,
    language:String,
    permalink:String
}
pub struct UselessFacts {}
#[async_trait]
impl SlashCommand for UselessFacts {
    async fn run(&self, _ctx: &Context, _interaction: &Interaction, common: &ScCommon) -> Option<Result<String, CommandError>> {
        let raw = common.http_client.get("https://uselessfacts.jsph.pl/api/v2/facts/random").send().await;
        if let Err(err) = raw {
            return Some(Err(CommandError::Default(format!("Failed to fetch: {err}").to_owned())))
        } 
        let res = raw.unwrap();
        let text_raw = res.json::<Fact>().await;
        if let Err(err) = text_raw {
            return Some(Err(CommandError::Default(format!("Failed to parse result: {err}").to_owned())))
        }
        let fact = text_raw.unwrap();
        Some(Ok(fact.text))
    }
    
    fn register(&self) -> CreateCommand {
        CreateCommand::new("useless-facts")
        .description("get some.. weird facts")
    }
    
    fn able_to_register(&self) -> bool {
        true
    }
}