use reqwest::Client as HttpClient;
use shuttle_runtime::{SecretStore};
pub use cmd_args_ext::CommandError;
pub struct Common {
    pub http_client:HttpClient
}