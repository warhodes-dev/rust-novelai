use serde::Deserialize;
use serde_json::Value;
use crate::{
    Result,
    error::Error,
    api::{URL, ErrorResponse},
};

pub mod subscription;

/// Returns: accessToken
pub async fn login(key: String) -> Result<String> {
    let res = reqwest::Client::new()
        .post(format!("{URL}/user/login"))
        .json(&serde_json::json!({
            "key": key,
        }))
        .send().await?;

    if res.status().is_success() {
        Ok(res.json::<Value>().await?["accessToken"].to_string())
    } else {
        let err = res.json::<ErrorResponse>().await?;
        Err(Error::Api(err))
    }
}
