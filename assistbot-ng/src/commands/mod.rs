pub mod util;
pub mod shared;
use cmd_args_ext::EnumArgsExt;
use shared::dicebear::StyleVariations;
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
        // for enum list, useful for Dicebear and others
        register_sc(slash::enum_list::EnumList {}),
        // ===
        register_sc(slash::useless_facts::UselessFacts {}),
        register_sc(slash::hello::Hello {}),
        register_sc(slash::ask::Ask {}),
        register_sc(slash::dicebear::Dicebear {}),
        register_sc(slash::imagine::Imagine {}),
        register_sc(slash::thanks::Thanks {})
    ]
);

pub static ENUMS: LazyLock<EnumType> = LazyLock::new(|| 
    vec![
        register_enum::<StyleVariations>()
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
fn register_enum<T: EnumArgsExt + Send + 'static>() -> (String, Vec<String>) {
    (T::enum_name(), T::to_vec())
}
pub type MbcType = Vec<Option<Box<dyn MessageBasedCommand>>>;
pub type ScType = Vec<Option<Box<dyn SlashCommand>>>;
pub type EnumType = Vec<(String, Vec<String>)>;