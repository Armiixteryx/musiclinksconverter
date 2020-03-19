use tbot::prelude::*;
use reqwest::header::{ACCEPT, X_CONTENT_TYPE_OPTIONS, AUTHORIZATION};

struct LinkData {
    artist: String,
    track: String,
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

async fn spotify_link(link: &str) -> Result<LinkData, reqwest::Error> {

    const API_LINK: &'static str = "https://api.spotify.com/v1/tracks/";

    const SPOTIFY_URL: &'static str = "https://open.spotify.com/track/";

    const SPOTIFY_ID_LEN: usize = 22;

    let (song, _) = link.trim_start_matches(SPOTIFY_URL).split_at(SPOTIFY_ID_LEN);

    let link = format!("{}{}", API_LINK, song);

    const TOKEN: &'static str = "";


    let client = reqwest::Client::new();
    let res = client
        .get(&link)
        .header(ACCEPT, "application/json")
        .header(X_CONTENT_TYPE_OPTIONS, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", TOKEN))
        .send()
        .await?;

    println!("status: {}", res.status().as_str());

    let res = res.text().await?;
    println!("request: {}", res);

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

            let spotify_data = spotify_link(&context.text.value).await.unwrap();

            let message = deezer_search_link(&spotify_data).await.unwrap();
            
            context.send_message_in_reply(&message).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();

    Ok(())
}
