use reqwest::Client as HttpClient;
use shuttle_runtime::{SecretStore};

pub struct Common {
    pub http_client:HttpClient
}
#[derive(Debug)]
pub enum CommandError {
    Default(String),
    Argument(String, String)
}