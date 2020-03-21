use tbot::prelude::*;
use reqwest::header::{ACCEPT, X_CONTENT_TYPE_OPTIONS, AUTHORIZATION};
use serde::Deserialize;
use std::env::var;

struct LinkData {
    artist: String,
    track: String,
}

#[derive(Deserialize, Debug)]
struct SpotifyAuth {
    access_token: String,
    token_type: String,
    expires_in: u32
}

struct SpotifyCredentials {
    client_id: String,
    client_secret: String
}

async fn deezer_search_link(song: &LinkData) -> Result<String, reqwest::Error> {

    const API: &'static str = "https://api.deezer.com/";
    const API_SEARCH: &'static str = "search?q=";
    
    let link = format!("{}{}artist:\"{}\" track:\"{}\"", API, API_SEARCH, song.artist, song.track);

    let res = reqwest::get(&link)
    .await?
    .text()
    .await?;

    let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

    dbg!(&res);

    let res = res["data"][0]["link"].as_str().unwrap();

    Ok(res.to_string())
}

async fn spotify_link(link: &str, credentials: &SpotifyCredentials) -> Result<LinkData, reqwest::Error> {

    let client = reqwest::Client::new();

    const DOMAIN: &'static str = "https://api.spotify.com";

    const API_VERSION: &'static str = "/v1";

    const API_TRACKS: &'static str = "/tracks/";

    const SPOTIFY_URL: &'static str = "https://open.spotify.com/track/";

    const SPOTIFY_ID_LEN: usize = 22;

    //Auth
    const TOKEN_URL: &'static str = "https://accounts.spotify.com/api/token";

    let credentials = base64::encode(format!("{}:{}", credentials.client_id, credentials.client_secret));

    let auth = format!("Basic {}", credentials);
    let body_params = [("grant_type", "client_credentials")];

    let token_res = client
        .post(TOKEN_URL)
        .header(AUTHORIZATION, auth)
        .form(&body_params)
        .send()
        .await?
        .json::<SpotifyAuth>()
        .await?;

    //let token_res = token_res.send().await?;

    //dbg!(&token_res);
    //End auth

    let (song, _) = link.trim_start_matches(SPOTIFY_URL).split_at(SPOTIFY_ID_LEN);

    let link = format!("{}{}{}{}", DOMAIN, API_VERSION, API_TRACKS, song);

    //const TOKEN: &'static str = "";
    
    let res = client
        .get(&link)
        .header(ACCEPT, "application/json")
        .header(X_CONTENT_TYPE_OPTIONS, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token_res.access_token))
        .send()
        .await?;

    println!("status: {}", res.status().as_str());

    let res = res.text().await?;
    //println!("request: {}", res);
    dbg!(&res);

    let res: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();

    let track = res["name"].as_str().unwrap().to_string();
    let artist = res["artists"][0]["name"].as_str().unwrap().to_string();

    Ok(
        LinkData {
        artist,
        track
    })
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {

            let spotify_credentials = SpotifyCredentials {
                client_id: match var("BOT_SPOTIFY_CLIENT_ID") {
                    Ok(var) => var,
                    Err(_) => panic!(),
                },
                client_secret: match var("BOT_SPOTIFY_CLIENT_SECRET") {
                    Ok(var) => var,
                    Err(_) => panic!(),
                }
            };

            let spotify_data = spotify_link(&context.text.value, &spotify_credentials).await.unwrap();

            let message = deezer_search_link(&spotify_data).await.unwrap();
            
            context.send_message_in_reply(&message).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();

    Ok(())
}
