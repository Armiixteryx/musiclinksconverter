use super::{UrlData, MusicType};

const API: &'static str = "https://api.deezer.com";
const API_SEARCH: &'static str = "/search?q=";

const DEEZER_ID_LEN: usize = 9;

pub const DEEZER_DOMAIN: &'static str = "https://www.deezer.com";

const ENDPOINT_TRACK: &'static str = "/track/";

pub struct DeezerController {
    client: reqwest::Client
}

impl DeezerController {
    pub fn new() -> DeezerController {
        DeezerController {
            client: reqwest::Client::new()
        }
    }

    pub async fn analyze_url(&self, url: &str) -> Result<UrlData, reqwest::Error> {
        let (id, _) = url.trim_start_matches(format!("{}{}", DEEZER_DOMAIN, ENDPOINT_TRACK).as_str()).split_at(DEEZER_ID_LEN);

        let req_link = format!("{}{}{}", API, ENDPOINT_TRACK, id);

        //dbg!(&req_link);

        let res = self.client
            .get(&req_link)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&res);

        let track = res["title"].as_str().unwrap();
        //dbg!(&track);

        let artist = res["artist"]["name"].as_str().unwrap();
        //dbg!(&artist);

        Ok(UrlData {
            artist: artist.to_string(),
            track: track.to_string()
        })
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