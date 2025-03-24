use reqwest::Client as HttpClient;
use serenity::all::{CreateEmbed};
pub use cmd_args_ext::CommandError;

use crate::settings;
pub struct Common {
    pub http_client:HttpClient
}

pub trait EmbedFromSettings {
    fn new_from_settings() -> Self;
}
impl EmbedFromSettings for CreateEmbed {
    fn new_from_settings() -> Self {
        CreateEmbed::new().color(settings::ACCENT_COLOR).author(settings::SET_EMBED_AUTHOR.clone())
    }
}