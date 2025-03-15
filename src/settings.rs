
use serenity::all::GatewayIntents;

// your bot permission. it woould be dynamically managed by under those settings
// btw, you need to add GUILD_MESSAGES if you enable message based commands
pub const INTENTS: [GatewayIntents; 1] = [GatewayIntents::GUILD_MESSAGES];

// disable the default `!run`
pub const DISABLE_RUN_FRAMEWORK:bool = false;

// disable all commands located on `commands/message`
pub const DISABLE_MESSAGE_BASED_COMMANDS:bool = false;

pub const MESSAGE_BASED_COMMANDS_PREFIX:&str = "asbt";
pub const ONLY_ALLOW_MESSAGE_BASED_COMMANDS_ON_CHANNELS: [&str; 1] = ["1348657323844833431"];