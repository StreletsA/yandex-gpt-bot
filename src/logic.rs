use std::collections::HashMap;

use reqwest::Error;

use tg::dto::{TgResponse, TgUpdate};

use crate::tg;
use crate::tg::api::send_message;
use crate::ya_gpt;

pub async fn start_main_loop() {
    let mut last_processed_update_id: i128 = -1;
    let mut last_processed_message_by_user: HashMap<i128, i128> = HashMap::new();

    loop {
        match get_response_with_updates(&last_processed_update_id).await {
            Some(response) => {
                let updates = response.result;
                if (updates.is_empty()) {
                    println!("No updates");
                    continue;
                }

                for update in updates {
                    process_tg_update(&update, &mut last_processed_message_by_user, &mut last_processed_update_id).await;
                }

                println!("last_processed_update_id: {last_processed_update_id},\
                last_processed_message_by_user: {last_processed_message_by_user:?}")
            },
            None => {
                println!("No response")
            }
        }
    }
}

async fn get_response_with_updates(last_processed_update_id: &i128) -> Option<TgResponse<Vec<TgUpdate>>> {
    match send_get_updates_request(last_processed_update_id).await {
        Ok(tg_response) => {
            Some(tg_response)
        },
        Err(e) => {
            println!("Error -> {e:?}");
            None
        }
    }
}

async fn send_get_updates_request(last_processed_update_id: &i128) -> Result<TgResponse<Vec<TgUpdate>>, Error> {
    if *last_processed_update_id == -1 {
        tg::api::get_all_updates_of_messages().await
    } else {
        tg::api::get_updates_of_messages(*last_processed_update_id + 1).await
    }
}

async fn process_tg_update(tg_update: &TgUpdate,
                           last_processed_message_by_user: &mut HashMap<i128, i128>,
                           last_processed_update_id: &mut i128) {
    println!("Processing {:?}", tg_update);

    let update_id = tg_update.update_id;
    if (update_id < *last_processed_update_id) {
        return;
    }

    let tg_message = &tg_update.message;
    let message_id = tg_message.message_id;
    let tg_user = &tg_message.from;
    let user_id = tg_user.id;

    let can_proceed: bool = match last_processed_message_by_user.get(&user_id) {
        Some(last_processed_message_id) => {
            *last_processed_message_id < message_id
        },
        None => true
    };

    if (!can_proceed) {
        return;
    }

    let text = &tg_message.text;
    match ya_gpt::api::send_prompt(text.clone()).await {
        Ok(ya_gpt_response) => {
            let ya_gpt_response_result = ya_gpt_response.result;
            let alternatives = ya_gpt_response_result.alternatives;
            if (alternatives.is_empty()) {
                println!("No answer from ya-gpt");
            } else {
                match alternatives.get(0) {
                    Some(alternative) => {
                        let ya_gpt_message = &alternative.message;
                        let ya_gpt_answer = &ya_gpt_message.text;
                        let chat_id = tg_message.chat.id;

                        match send_message(chat_id, ya_gpt_answer.clone()).await {
                            Ok(_) => {
                                println!("Answer from YaGPT {ya_gpt_answer} sent to chat {chat_id}")
                            }
                            Err(_) => {
                                println!("No answer from ya-gpt: no alternative")
                            }
                        }
                    },
                    None => {
                        println!("No answer from ya-gpt: no alternative");
                    }
                }
            }
        },
        Err(e) => {
            println!("Error until sending prompt: {e:?}");
        }
    }

    *last_processed_update_id = update_id;
    last_processed_message_by_user.insert(user_id, message_id);
}