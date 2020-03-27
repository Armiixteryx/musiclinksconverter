use super::{UrlData, Constants};
use url::Url;

const API: &'static str = "https://api.deezer.com";

const API_SEARCH: &'static str = "/search";

const ENDPOINT_TRACK: &'static str = "/track/";

pub struct Metadata {}

impl<'a> Constants<'a> for Metadata {
    const URL_PLAYER_HOST: &'static str = "www.deezer.com";
    
    fn music_object_type() -> Vec<&'a str> {
        vec!["track", "album", "artist"]
    }

    const ID_LEN: usize = 9;
}

pub struct DeezerController {
    client: reqwest::Client
}

impl DeezerController {
    pub fn new() -> DeezerController {
        DeezerController {
            client: reqwest::Client::new()
        }
    }

    pub async fn analyze_url(&self, id: &str) -> Result<UrlData, reqwest::Error> {
        let mut request = Url::parse(API).unwrap();

        request.set_path(&format!("{}{}", ENDPOINT_TRACK, id));

        //dbg!(&request);

        let response = self.client
            .get(request.as_str())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&response);

        let track = response["title"].as_str().unwrap();
        //dbg!(&track);

        let artist = response["artist"]["name"].as_str().unwrap();
        //dbg!(&artist);

        Ok(UrlData {
            artist: artist.to_string(),
            track: track.to_string()
        })
    }

    pub async fn generate_url(&self, data: &UrlData) -> Result<String, reqwest::Error> {
        let mut request = Url::parse(API).unwrap();
        request.set_path(API_SEARCH);
        
        let request_query = format!("artist:\"{}\" track:\"{}\"", data.artist, data.track);

        let response = self.client.get(request.as_str())
            .query(&[("q", request_query.as_str())])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&response);

        let response = response["data"][0]["link"].as_str().unwrap();

        Ok(response.to_string())
    }
}