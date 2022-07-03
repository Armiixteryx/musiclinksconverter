use super::{Constants, UrlData};
//use reqwest::header::{ACCEPT, AUTHORIZATION, X_CONTENT_TYPE_OPTIONS};
//use serde::Deserialize;
use std::env::var;
use url::Url;

const API: &'static str = "https://api.spotify.com";

const ENDPOINT_VERSION: &'static str = "/v1";

const ENDPOINT_TRACKS: &'static str = "/tracks/";

const ENDPOINT_SEARCH: &'static str = "/search";

const TOKEN_URL: &'static str = "https://accounts.spotify.com/api/token";

const ENV_CLIENT_ID: &'static str = "BOT_SPOTIFY_CLIENT_ID";
const ENV_CLIENT_SECRET: &'static str = "BOT_SPOTIFY_CLIENT_SECRET";

pub struct Metadata {}

impl<'a> Constants<'a> for Metadata {
    const URL_PLAYER_HOST: &'static str = "open.spotify.com";

    fn music_object_type() -> Vec<&'a str> {
        vec!["track", "album", "artist"]
    }

    const ID_LEN: usize = 22;
}

/*
#[derive(Deserialize, Debug)]
struct SpotifyAuth {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

#[derive(Debug)]
pub struct SpotifyCredentials {
    pub client_id: String,
    pub client_secret: String,
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
            client_secret: var(ENV_CLIENT_SECRET).expect("Secret ID not found."),
        };

        let b64_credentials = base64::encode(format!(
            "{}:{}",
            credentials.client_id, credentials.client_secret
        ));

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
            auth_token,
        })
    }

    pub async fn analyze_url(&self, id: &str) -> Result<UrlData, reqwest::Error> {
        let mut request = Url::parse(API).unwrap();
        request.set_path(&format!("{}{}{}", ENDPOINT_VERSION, ENDPOINT_TRACKS, id));

        //dbg!(&url);

        let response = self
            .http_client
            .get(request.as_str())
            .header(ACCEPT, "application/json")
            .header(X_CONTENT_TYPE_OPTIONS, "application/json")
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.auth_token.access_token),
            );

        //dbg!(&response);

        let response = response.send().await?;

        //dbg!(&response);

        let response = response.json::<serde_json::Value>().await?;

        //dbg!(&response);

        let track = response["name"].as_str().unwrap().to_string();
        let artist = response["artists"][0]["name"].as_str().unwrap().to_string();

        Ok(UrlData { artist, track })
    }

    pub async fn generate_url(&self, data: &UrlData) -> Result<String, reqwest::Error> {
        let mut request = Url::parse(API).unwrap();
        request.set_path(&format!("{}{}", ENDPOINT_VERSION, ENDPOINT_SEARCH));

        let data = format!("artist:\"{}\" track:\"{}\"", data.artist, data.track);
        let query_data = ("q", data.as_str());
        let query_type = ("type", "track");
        let query_limit = ("limit", "1");

        let list = [query_data, query_type, query_limit];

        let res = self
            .http_client
            .get(request.as_str())
            .header(ACCEPT, "application/json")
            .header(X_CONTENT_TYPE_OPTIONS, "application/json")
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.auth_token.access_token),
            )
            .query(&list)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&res);

        let id = res["tracks"]["items"][0]["id"].as_str().unwrap();

        Ok(format!(
            "https://{}/track/{}",
            Metadata::URL_PLAYER_HOST,
            id
        ))
    }
}
*/
