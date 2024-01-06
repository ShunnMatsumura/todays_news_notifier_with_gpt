use reqwest;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Debug)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: [Message; 2],
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok(); // .envファイルから環境変数を読み込む
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    // let prompt = "Tell me the latest news headlines for today, please.";

    let response = get_response_from_openai(&api_key).await?;
    println!("Response from OpenAI: {}", response);

    Ok(())
}

async fn get_response_from_openai(api_key: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let request_body = OpenAIRequest {
        model: "gpt-3.5-turbo",
        messages: [
            Message {
                role: "system".to_string(),
                content: "Hello!".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "Oh!".to_string(),
            },
        ]
    };

    let response = client
        .post(OPENAI_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    println!("Response status: {:#?}", response);

    let response_body: OpenAIResponse = response.json().await?;
    println!("{:#?}", response_body);
    let text = response_body.choices.get(0).map_or("", |choice| &choice.message.content);

    Ok(text.to_string())
}