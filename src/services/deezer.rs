use super::{Constants, UrlData};
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

#[derive(Debug, serde::Deserialize)]
struct ServerResponse {
    data: serde_json::Value,
    total: usize
}

#[derive(Debug)]
pub enum DeezerError {
    NotFound,
    Reqwest(reqwest::Error),
    Deserialization(serde_json::error::Error),
    Other(String)
}

impl From<reqwest::Error> for DeezerError {
    fn from(err: reqwest::Error) -> Self {
        DeezerError::Reqwest(err)
    }
}

impl From<serde_json::error::Error> for DeezerError {
    fn from(err: serde_json::error::Error) -> Self {
        DeezerError::Deserialization(err)
    }
}

impl From<url::ParseError> for DeezerError {
    fn from(err: url::ParseError) -> Self {
        DeezerError::Other(err.to_string())
    }
}

pub struct DeezerController {
    client: reqwest::Client,
}

impl DeezerController {
    pub fn new() -> DeezerController {
        DeezerController {
            client: reqwest::Client::new(),
        }
    }

    pub async fn analyze_url(&self, id: &str) -> Result<UrlData, DeezerError> {
        let mut request = Url::parse(API)?;

        request.set_path(&format!("{}{}", ENDPOINT_TRACK, id));

        let response = self
            .client
            .get(request.as_str())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let track = match response["title"].as_str() {
            Some(track) => track.to_string(),
            None => return Err(DeezerError::Other("The data extraction returned nothing".to_string()))
        };

        let artist = match response["artist"]["name"].as_str() {
            Some(artist) => artist.to_string(),
            None => return Err(DeezerError::Other("The data extraction returned nothing".to_string()))
        };

        Ok(UrlData {
            artist,
            track,
        })
    }

    pub async fn generate_url(&self, data: &UrlData) -> Result<String, DeezerError> {
        let mut request = Url::parse(API)?;
        request.set_path(API_SEARCH);

        let request_query = format!("artist:\"{}\" track:\"{}\"", data.artist, data.track);

        let response = self
            .client
            .get(request.as_str())
            .query(&[("q", request_query.as_str())])
            .send()
            .await?;
            
        // TODO: Use status to handle errors
        //dbg!(&response.status());

        let response: ServerResponse = serde_json::from_slice(&response.bytes().await?)?;

        let res_total: usize = response.total;

        // It's a strange thing that when Deezer doesn't find a song
        // it returns a void request rather than ERR 800 of its API
        if res_total == 0 {
            return Err(DeezerError::NotFound);
        }

        match response.data[0]["link"].as_str() {
            Some(res) => Ok(res.to_string()),
            None => Err(DeezerError::Other("The data extraction returned nothing".to_string()))
        }

        //Ok(response.to_string())
    }
}
