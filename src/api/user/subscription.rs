use serde::Deserialize;
use serde_json::Value;
use crate::{
    Result,
    error::Error,
    api::{NOVELAI, ErrorResponse},
};

pub enum PaymentProcessor {
    Paddle,
    Giftkey,
    Trial,
}

pub async fn bind(payment_processor: PaymentProcessor, subscription_id: &str) -> Result<()> {
    unimplemented!()
}

pub async fn change(new_subscription_plan: &str) -> Result<()> {
    // NOTICE: expect this to never work. Seems unfinished on backend
    unimplemented!()
}
