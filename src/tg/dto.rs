use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageQuery {
    pub chat_id: i128,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUpdatesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgUpdate {
    pub update_id: i128,
    pub message: TgMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgMessage {
    pub message_id: i128,
    pub from: TgUser,
    pub chat: TgChat,
    pub text: String,
    pub date: i128
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgUser {
    pub id: i128,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TgChat {
    pub id: i128,
    #[serde(alias = "type")]
    pub chat_type: String,
}