use super::{UrlData};
use serde::Deserialize;
use std::env::var;
use reqwest::header::{ACCEPT, X_CONTENT_TYPE_OPTIONS, AUTHORIZATION};

const DOMAIN: &'static str = "https://api.spotify.com";

const API_VERSION: &'static str = "/v1";

const API_TRACKS: &'static str = "/tracks/";

const API_SEARCH: &'static str = "/search";

pub const SPOTIFY_DOMAIN: &'static str = "https://open.spotify.com";

const SPOTIFY_URL: &'static str = "https://open.spotify.com/track/";

const SPOTIFY_ID_LEN: usize = 22;

const TOKEN_URL: &'static str = "https://accounts.spotify.com/api/token";

const ENV_CLIENT_ID: &'static str = "BOT_SPOTIFY_CLIENT_ID";
const ENV_CLIENT_SECRET: &'static str = "BOT_SPOTIFY_CLIENT_SECRET";

#[derive(Deserialize, Debug)]
struct SpotifyAuth {
    access_token: String,
    token_type: String,
    expires_in: u32
}

#[derive(Debug)]
pub struct SpotifyCredentials {
    pub client_id: String,
    pub client_secret: String
}

pub struct SpotifyController {
    http_client: reqwest::Client,
    credentials: SpotifyCredentials,
    auth_token: SpotifyAuth,
}

impl SpotifyController {
    pub async fn new() -> Result<SpotifyController, reqwest::Error> {
        let http_client = reqwest::Client::new();
        
        let credentials = SpotifyCredentials {
            client_id: var(ENV_CLIENT_ID).expect("Client ID not found."),
            client_secret: var(ENV_CLIENT_SECRET).expect("Secret ID not found.")
        };

        let b64_credentials = base64::encode(format!("{}:{}", credentials.client_id, credentials.client_secret));

        let auth_params = format!("Basic {}", b64_credentials);
        let body_params = [("grant_type", "client_credentials")];

        let auth_token = http_client
            .post(TOKEN_URL)
            .header(AUTHORIZATION, auth_params)
            .form(&body_params)
            .send()
            .await?
            .json::<SpotifyAuth>()
            .await?;

        //dbg!(&credentials);
        //dbg!(&auth_token);

        Ok(SpotifyController {
            http_client,
            credentials,
            auth_token
        })
    }

    pub async fn analyze_url(&self, url: &str) -> Result<UrlData, reqwest::Error> {
        let (url, _) = url.trim_start_matches(SPOTIFY_URL).split_at(SPOTIFY_ID_LEN);

        let url = format!("{}{}{}{}", DOMAIN, API_VERSION, API_TRACKS, url);

        //dbg!(&url);

        let res = self.http_client
            .get(&url)
            .header(ACCEPT, "application/json")
            .header(X_CONTENT_TYPE_OPTIONS, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token.access_token))
            .send()
            .await?;
        
        //dbg!(&res);

        let res = res
            .text()
            .await?;

        //dbg!(&res);

        let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

        let track = res["name"].as_str().unwrap().to_string();
        let artist = res["artists"][0]["name"].as_str().unwrap().to_string();
        
        Ok(UrlData {
            artist,
            track
        })
    }
    
    pub async fn generate_url(&self, data: &UrlData) -> Result<String, reqwest::Error> {
        let data = format!("artist:\"{}\" track:\"{}\"", data.artist, data.track);
        let query_data = ("q", data.as_str());
        let query_type = ("type", "track");
        let query_limit = ("limit", "1");

        let list = [query_data, query_type, query_limit];

        let link = format!("{}{}{}", DOMAIN, API_VERSION, API_SEARCH);

        let res = self.http_client
            .get(&link)
            .header(ACCEPT, "application/json")
            .header(X_CONTENT_TYPE_OPTIONS, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token.access_token))
            .query(&list)
            .send()
            .await?;

        //dbg!(&res);

        let res = res
            .json::<serde_json::Value>()
            .await?;
        
        //dbg!(&res);

        let id = res["tracks"]["items"][0]["id"].as_str().unwrap();

        Ok(format!("{}{}", SPOTIFY_URL, id))
    }
}