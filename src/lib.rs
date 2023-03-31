pub mod api;
pub mod utils;
mod error;

type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug)]
pub struct Client {
    access_token: String,
}

impl Client {
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        let api_key = utils::get_access_key(email, password)?;
        let api_token = api::user::login(api_key).await?;
        Ok(Client {
            access_token: api_token,
        })
    }
}
