use serde::Deserialize;
use serde_json::Value;
use crate::{
    Result,
    error::Error,
    api::{NOVELAI, ErrorResponse},
};

pub mod subscription;

pub async fn login(key: String) -> Result<String> {
    let res = reqwest::Client::new()
        .post(format!("{NOVELAI}/user/login"))
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
