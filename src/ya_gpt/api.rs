use reqwest;

use crate::ya_gpt::dto::{YaGPTCompletionOptions, YaGPTRequest, YaGPTRequestMessage, YaGPTResponse};

const TOKEN: &str = "t1.9euelZrKkMmRksqQisuenJ6Nx5DMk-3rnpWaj5CVi5fJmImSlczKlMiTjpPl8_cAd1lM-e9XSSAZ_t3z90AlV0z571dJIBn-zef1656VmoyVi5zPi82VyI-bi5SOmsuL7_zF656VmoyVi5zPi82VyI-bi5SOmsuL.esST6o9vx4Rzg3L54cH2iFK-Tu_XMn-s4NunhkgXei0K2b3McctUz-13vBnfaBBe8ZGeFtz-hiSVRLxmKPDVDg";
const CATALOG_TOKEN: &str = "b1gn4l28jnq32o2biaqp";
const CLOUD_ID: &str = "b1gqtp43bmvupn90cgmg";


// const LITE_MODEL_URI: &str = format!("gpt://{}/yandexgpt-lite/latest", CATALOG_TOKEN).as_str();
const CHAT_URL: &str = "https://llm.api.cloud.yandex.net/foundationModels/v1/completion";

pub async fn send_prompt(prompt: String) -> Result<YaGPTResponse, String> {
    let ya_gpt_request: YaGPTRequest = YaGPTRequest {
        token: TOKEN.parse().unwrap(),
        model_uri: format!("gpt://{}/yandexgpt/latest", CATALOG_TOKEN),
        completion_options: create_default_completion_options(),
        messages: vec![create_message(prompt)],
    };

    let client = reqwest::Client::new();

    let resp_text = match client.post(CHAT_URL)
        .header("Authorization", format!("Bearer {}", TOKEN))
        .header("Content-Type", "application/json")
        .json(&ya_gpt_request)
        .send()
        .await {
        Ok(response) => {
            match response.text().await {
                Ok(text) => {text}
                Err(e) => {
                    return Err(format!("Error until receiving text of response {e:?}"))
                }
            }
        }
        Err(e) => {
            return Err(format!("Error until receiving response {e:?}"))
        }
    };

    let unmarshalling_result = serde_json::from_str(&*resp_text);
    match unmarshalling_result {
        Ok(ya_gpt_response) => {Ok(ya_gpt_response)}
        Err(e) => {
            Err(format!("Error until unmarshall response: {} -> {e:?}", resp_text))
        }
    }
}

fn create_default_completion_options() -> YaGPTCompletionOptions {
    YaGPTCompletionOptions {stream: false, temperature: 0.3, max_tokens: 2000}
}

fn create_message(prompt: String) -> YaGPTRequestMessage {
    YaGPTRequestMessage {role: "user".parse().unwrap(), text: prompt}
}