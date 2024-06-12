use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTRequest {
    pub token: String,
    #[serde(alias = "modelUri")]
    pub model_uri: String,
    #[serde(alias = "completionOptions")]
    pub completion_options: YaGPTCompletionOptions,
    pub messages: Vec<YaGPTRequestMessage>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTCompletionOptions {
    pub stream: bool,
    pub temperature: f32,
    #[serde(alias = "maxTokens")]
    pub max_tokens: i128
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTRequestMessage {
    pub role: String,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTResponse {
    pub result: YaGPTResponseResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTResponseResult {
    pub alternatives: Vec<YaGPTResponseAlternative>,
    pub usage: YaGPTResponseUsage,
    #[serde(alias = "modelVersion")]
    pub model_version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTResponseUsage {
    #[serde(alias = "inputTextTokens")]
    pub input_text_tokens: String,
    #[serde(alias = "completionTokens")]
    pub completion_tokens: String,
    #[serde(alias = "totalTokens")]
    pub total_tokens: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTResponseAlternative {
    pub message: YaGPTResponseAlternativeMessage,
    pub status: YaGPTResponseAlternativeStatus
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YaGPTResponseAlternativeMessage {
    pub role: String,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum YaGPTResponseAlternativeStatus {
    #[serde(alias = "ALTERNATIVE_STATUS_UNSPECIFIED")]
    AlternativeStatusUnspecified,
    #[serde(alias = "ALTERNATIVE_STATUS_PARTIAL")]
    AlternativeStatusPartial,
    #[serde(alias = "ALTERNATIVE_STATUS_TRUNCATED_FINAL")]
    AlternativeStatusTruncatedFinal,
    #[serde(alias = "ALTERNATIVE_STATUS_FINAL")]
    AlternativeStatusFinal,
    #[serde(alias = "ALTERNATIVE_STATUS_CONTENT_FILTER")]
    AlternativeStatusContentFilter
}