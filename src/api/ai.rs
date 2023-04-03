use serde::{Serialize, Deserialize};
use serde_with_macros::skip_serializing_none;
use crate::{
    api::{URL, ErrorResponse},
    Result, 
    error::Error
};

pub async fn generate(
    input: &str, 
    model: &AiModel, 
    params: &AiGenerateParameters
) -> Result<AiGenerateResponse> {

    if params.min_length < 1.0 || params.min_length > 2048.0 {
        return Err(Error::BadArguments("AI Generation parameter 'min_length' must be between 1 and 2048".to_string()));
    } else if params.max_length < 1.0 || params.max_length > 2048.0 {
        return Err(Error::BadArguments("AI Generation parameter 'max_length' must be between 1 and 2048".to_string()));
    }

    let res = reqwest::Client::new()
        .post(format!("{URL}/ai/generate"))
        .json(&serde_json::json!({
            "input": input,
            "model": model.as_str(),
            "parameters": params,
        }))
        .send().await?;

    if res.status().is_success() {
        Ok(res.json::<AiGenerateResponse>().await?)
    } else {
        let err = res.json::<ErrorResponse>().await?;
        Err(Error::Api(err))
    }
}

#[derive(Debug, Deserialize)]
pub struct AiGenerateResponse {
    pub output: Option<String>,
    pub error: Option<String>,
}

pub enum AiModel {
    _27B,
    _6Bv4,
    Euterpe,
    Genji,
    GenjiJP,
    GenjiJPv2,
    Krake,
    Hypebot,
    Infillmodel,
}

impl AiModel {
    fn as_str(&self) -> &'static str {
        match self {
            AiModel::_27B => "2.7B",
            AiModel::_6Bv4 => "6B-v4",
            AiModel::Euterpe => "euterpe-v2",
            AiModel::Genji => "genji-python-6b",
            AiModel::GenjiJP => "genji-jp-6b",
            AiModel::GenjiJPv2 => "genji-jp-6b-v2",
            AiModel::Krake => "krake-v2",
            AiModel::Hypebot => "hypebot",
            AiModel::Infillmodel => "infillmodel",
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AiGenerateParameters {
    pub min_length: f32,
    pub max_length: f32,

    pub stop_sequences: Option<Vec<Vec<i32>>>,
    pub bad_words_ids: Option<Vec<Vec<i32>>>,
    pub logit_bias: Option<Vec<Vec<i32>>>,
    pub logit_bias_exp: Option<Vec<LogitBiasExp>>,
    pub order: Option<Vec<i32>>,
    pub repetition_penalty_whitelist: Option<Vec<i32>>,
    pub temperature: Option<f32>,
    pub do_sample: Option<bool>,
    pub early_stopping: Option<bool>,
    pub num_beams: Option<f32>,
    pub top_k: Option<f32>,
    pub top_a: Option<f32>,
    pub top_p: Option<f32>,
    pub typical_p: Option<f32>,
    pub repetition_penalty: Option<f32>,
    pub pad_token_id: Option<f32>,
    pub bos_token_id: Option<f32>,
    pub eos_token_id: Option<f32>,
    pub length_penalty: Option<f32>,
    pub no_repeat_ngram_size: Option<f32>,
    pub encoder_no_repeat_ngram_size: Option<f32>,
    pub num_return_sequences: Option<f32>,
    pub max_time: Option<f32>,
    pub use_cache: Option<bool>,
    pub num_beam_groups: Option<f32>,
    pub diversity_penalty: Option<f32>,
    pub tail_free_sampling: Option<f32>,
    pub repetition_penalty_range: Option<f32>,
    pub repetition_penalty_slope: Option<f32>,
    pub get_hidden_states: Option<bool>,
    pub repetition_penalty_frequency: Option<f32>,
    pub repetition_penalty_presence: Option<f32>,
    pub next_word: Option<bool>,
    pub prefix: Option<String>,
    pub output_nonzero_probs: Option<bool>,
    pub generate_until_sentence: Option<bool>,
    pub num_logprobs: Option<f32>,

    /// If false, input and output strings should be Base64-encoded uint16 numbers representing tokens
    pub use_string: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogitBiasExp {
    pub sequence: Vec<i32>,
    pub bias: f32,
    pub ensure_sequence_finish: Option<bool>,
    pub generate_once: Option<bool>,
}


