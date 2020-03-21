use super::{UrlData, MusicType};

const API: &'static str = "https://api.deezer.com/";
const API_SEARCH: &'static str = "search?q=";

pub struct DeezerController {
    client: reqwest::Client
}

impl DeezerController {
    pub fn new() -> DeezerController {
        DeezerController {
            client: reqwest::Client::new()
        }
    }

    pub async fn analyze_url(&self, url: &str) -> UrlData {
        unimplemented!();
    }

    pub async fn generate_url(&self, data: &UrlData) -> Result<String, reqwest::Error> {
        let link = format!("{}{}artist:\"{}\" track:\"{}\"", API, API_SEARCH, data.artist, data.track);

        let res = self.client.get(&link)
            .send()
            .await?
            .text()
            .await?;
        
        let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

        let res = res["data"][0]["link"].as_str().unwrap();

        Ok(res.to_string())
    }
}