use serde::Deserialize;
use serde_json;
use std::{env, error::Error, fmt};
use tracing::{debug, warn};

#[derive(Debug)]
pub struct ProvidedImage {
    pub url: String,
}

#[derive(Debug)]
struct ImageProviderError(String);
impl fmt::Display for ImageProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ImageProviderError {}

#[derive(Deserialize, Debug)]
struct NekosBestResult {
    anime_name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct NekosBestJson {
    results: Vec<NekosBestResult>,
}

#[derive(Deserialize, Debug)]
struct SafebooruResult {
    file_url: String,
    id: i32,
}

pub async fn nekos_best(query: &str) -> Result<ProvidedImage, Box<dyn Error + Send + Sync>> {
    let endpoint = env::var("NEKO_ENDPOINT").unwrap_or("https://nekos.best/api/v2/".to_string());
    let url = format!("{}{}", endpoint, query);

    let client = reqwest::Client::builder()
        .user_agent("discordbot")
        .build()?;
    let response = client.get(&url).send().await?;

    let response_text = response.text().await?;
    let res: NekosBestJson = serde_json::from_str(&response_text).map_err(|e| {
        Box::new(ImageProviderError(format!(
            "JSON parse error: {}. Raw response: {}",
            e, response_text
        )))
    })?;

    Ok(ProvidedImage {
        url: res.results.first().unwrap().url.clone(),
    })
}

pub async fn safebooru(tags: &str) -> Result<ProvidedImage, Box<dyn Error + Send + Sync>> {
    let endpoint = env::var("SAFEBOORU_ENDPOINT").unwrap_or(
        "https://safebooru.org/index.php?page=dapi&s=post&q=index&json=1&limit=1&tags=".to_string(),
    );
    let url = format!("{}{}", endpoint, tags.replace(" ", "+"));

    let client = reqwest::Client::builder()
        .user_agent("discordbot")
        .build()?;
    let response = client.get(&url).send().await?;

    let response_text = response.text().await?;
    let res: Vec<SafebooruResult> = serde_json::from_str(&response_text).map_err(|e| {
        Box::new(ImageProviderError(format!(
            "JSON parse error: {}. Raw response: {}",
            e, response_text
        )))
    })?;
    Ok(ProvidedImage {
        url: res.first().unwrap().file_url.clone(),
    })
}
