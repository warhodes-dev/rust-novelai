use serde::Deserialize;
use serde_json::Value;

use crate::Result;
use crate::error::Error;

pub mod user;
pub mod ai;

const NOVELAI: &str = "https://api.novelai.net";

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "statusCode")]
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for ErrorResponse {}

pub async fn status() -> Result<()> {
    let res = reqwest::Client::new()
        .get(format!("{NOVELAI}/"))
        .send().await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Error::UnknownError)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::get_access_key;

    use super::*;
    use super::user::*;

    #[tokio::test]
    async fn is_reachable() {
        let status = status().await;
        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn fail_login() {
        let key = get_access_key("warhodes@gmail.com", "blahblahblah").unwrap();
        let weenie = login(key).await;
        println!("weenie: {:?}", weenie);
    }
}
