pub mod util;
pub mod shared;
use util::{message::MessageBasedCommand, slash::{SlashCommand}};
use std::sync::LazyLock;
mod message;
mod slash;

pub static MESSAGE_BASED_COMMANDS: LazyLock<MbcType> = LazyLock::new(|| 
    vec![
        register_mbc(message::Run {})
    ]
);
pub static SLASH_COMMANDS: LazyLock<ScType> = LazyLock::new(|| 
    vec![
        register_sc(slash::useless_facts::UselessFacts {}),
        register_sc(slash::hello::Hello {}),
        register_sc(slash::ask::Ask {}),
        register_sc(slash::dicebear::Dicebear {})
    ]
);


fn register_mbc<T: MessageBasedCommand + Send + Sync + 'static>(cmd: T) -> Option<Box<dyn MessageBasedCommand>> {
    if cmd.able_to_register() {
        Some(Box::new(cmd))
    } else {
        None
    }
}
fn register_sc<T: SlashCommand + Send + Sync + 'static>(cmd: T) -> Option<Box<dyn SlashCommand>> {
    if cmd.able_to_register() {
        Some(Box::new(cmd))
    } else {
        None
    }
}
pub type MbcType = Vec<Option<Box<dyn MessageBasedCommand>>>;
pub type ScType = Vec<Option<Box<dyn SlashCommand>>>;