use reqwest;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use rss::Channel;
use std::error::Error;
use std::fmt;
use warp::Filter;

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

const RSS_FEEDS: [&str; 15] = [
    "https://www.searchenginejournal.com/feed/",
    "https://moz.com/blog/feed",
    "https://css-tricks.com/feed/",
    "https://frontendfoc.us/rss",
    "https://thenewstack.io/feed/",
    "https://feeds.dzone.com/deployment",
    "https://feeds.dzone.com/databases",
    "https://github.blog/feed/",
    "https://devops.com/feed/",
    "https://blog.cloudflare.com/rss/",
    "https://engineeringmanagementinstitute.org/feed/",
    "https://leaddev.com/content-piece-and-series/rss.xml",
    "https://feeds.feedburner.com/TheHackersNews",
    "https://krebsonsecurity.com/feed/",
    "https://techcrunch.com/feed/"
];

#[derive(Debug)]
enum NewsError {
    Reqwest(reqwest::Error),
    Rss(rss::Error),
    Other(String),
}

impl fmt::Display for NewsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NewsError::Reqwest(ref err) => err.fmt(f),
            NewsError::Rss(ref err) => err.fmt(f),
            NewsError::Other(ref err) => f.write_str(err),
        }
    }
}

impl Error for NewsError {}

impl From<reqwest::Error> for NewsError {
    fn from(err: reqwest::Error) -> NewsError {
        NewsError::Reqwest(err)
    }
}

impl From<rss::Error> for NewsError {
    fn from(err: rss::Error) -> NewsError {
        NewsError::Rss(err)
    }
}

impl From<&str> for NewsError {
    fn from(err: &str) -> NewsError {
        NewsError::Other(err.to_string())
    }
}

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
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");

    // summariesを取得するエンドポイント
    let summaries = warp::path("summaries")
    .and(warp::get())
    .and_then(move || get_summaries(api_key.clone()));

    // warpサーバーを起動
    warp::serve(summaries)
    .run(([127, 0, 0, 1], 3030))
    .await;
}

async fn get_summaries(api_key: String) -> Result<impl warp::Reply, warp::Rejection> {
    match get_latest_news_summaries(&api_key).await {
        Ok(summaries) => Ok(warp::reply::json(&summaries)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn get_latest_news_summaries(api_key: &str) -> Result<Vec<String>, NewsError> {
    let client = reqwest::Client::new();
    let mut summaries = Vec::new();

    for &feed in RSS_FEEDS.iter() {
        println!("Fetching feed: {}", feed);
        let response = client.get(feed).send().await.map_err(NewsError::from)?;
        let body = response.text().await.map_err(NewsError::from)?;
        let channel = Channel::read_from(body.as_bytes()).map_err(NewsError::from)?;
        let latest = channel.items().first().ok_or_else(|| NewsError::from("No items in feed"))?;
        let description = latest.description().ok_or_else(|| NewsError::from("No description in item"))?;
        let url = latest.link().ok_or_else(|| NewsError::from("No url in item"))?;
        let trimmed_description = &description[0..description.chars().take(1000).count()];
        let summary = summarize_news(&client, &api_key, trimmed_description, url).await.map_err(NewsError::from)?;
        summaries.push(summary);
    }

    Ok(summaries)
}

async fn summarize_news(client: &reqwest::Client, api_key: &str, news: &str, url: &str) -> Result<String, reqwest::Error> {
    // println!("Summarizing news: {}", news);
    let request_body = OpenAIRequest {
        model: "gpt-3.5-turbo",
        messages: [
            Message {
                role: "system".to_string(),
                content: "You are a translator. Your job is to summarize English news in Japanese in 100 characters and article url.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: news.to_string() + "\n" + url,
            },
        ],
    };

    let response = client
        .post(OPENAI_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // println!("Response: {:?}", response);

    let response_body: OpenAIResponse = response.json().await?;
    // println!("{:?}", response_body);
    let summary = response_body.choices.get(0).map_or(String::new(), |choice| choice.message.content.clone());

    Ok(summary)
}