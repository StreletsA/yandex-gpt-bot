use reqwest::{Error, Response};

use crate::tg::dto::{GetUpdatesQuery, SendMessageQuery, TgResponse, TgUpdate};

const TG_BOT_TOKEN: &str = "7122843724:AAEfz-vE3Fy6JQ0KBjfTABBYmhr0pju5T2M";
const TG_API_FORMAT: &str = "https://api.telegram.org/bot";
const SEND_MESSAGE: &str = "sendMessage";
const GET_UPDATES: &str = "getUpdates";

pub async fn get_all_updates_of_messages() -> Result<TgResponse<Vec<TgUpdate>>, Error> {
    let get_updates_query = GetUpdatesQuery {
        offset: None,
        limit: None,
        timeout: None,
        allowed_updates: Some(vec!["message".to_string()])
    };

    send_get_updates_request(get_updates_query).await
}

pub async fn get_updates_of_messages(offset: i128) -> Result<TgResponse<Vec<TgUpdate>>, Error> {
    let get_updates_query = GetUpdatesQuery {
        offset: Some(offset),
        limit: None,
        timeout: None,
        allowed_updates: Some(vec!["message".to_string()])
    };

    send_get_updates_request(get_updates_query).await
}

async fn send_get_updates_request(get_updates_query: GetUpdatesQuery) -> Result<TgResponse<Vec<TgUpdate>>, Error> {
    let client = reqwest::Client::new();
    let resp_text = client.post(create_method_url(GET_UPDATES))
        .json(&get_updates_query)
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str(&*resp_text) {
        Ok(r) => {Ok(r)},
        Err(e) => {panic!("Error -> {e:?}")},
    }
}

pub async fn send_message(chat_id: i128, text: String) -> Result<Response, Error> {
    let send_message_query = SendMessageQuery { chat_id, text };
    let client = reqwest::Client::new();
    client.post(create_method_url(SEND_MESSAGE))
        .json(&send_message_query)
        .send()
        .await
}

fn create_method_url(method: &str) -> String {
    format!("{}{}/{}", TG_API_FORMAT, TG_BOT_TOKEN, method)
}
