mod services;

use std::error::Error;
use tbot::prelude::*;
use tbot::types::chat::Action;

use services::{deezer::{DeezerError, DeezerController}, spotify::SpotifyController, Services};
//use services::UrlService;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {
            let user_msg = &context.text.value;

            let message: String = match services::get_service(user_msg) {
                Ok(url) => {
                    let service = url.service;
                    let id = url.id;

                    //dbg!(&service);

                    let control_deezer = DeezerController::new();
                    let control_spotify = SpotifyController::new().await.unwrap();

                    // `Typing...` status in chat
                    context
                        .send_chat_action(Action::Typing)
                        .call()
                        .await
                        .unwrap();

                    match service {
                        Services::Deezer => {
                            let deezer_data = control_deezer.analyze_url(&id).await.unwrap();

                            control_spotify.generate_url(&deezer_data).await.unwrap()
                        }
                        Services::Spotify => {
                            let spotify_data = control_spotify.analyze_url(&id).await.unwrap_or_else(|e| {
                                eprintln!("Error: {}", e);
                                eprintln!("Caused by: {}", e.source().unwrap());
                                panic!();
                            });

                            control_deezer.generate_url(&spotify_data).await.unwrap_or_else(|e| {
                                match e {
                                    DeezerError::NotFound => "Not found in Deezer.".to_string(),
                                    DeezerError::Reqwest(net_err) => format!("Network error: {}", net_err),
                                    DeezerError::Deserialization(_) => "Internal error".to_string(),
                                    DeezerError::Other(e) => {
                                        eprintln!("{}", e);
                                        "Internal error".to_string()
                                    }
                                }
                            })
                        }
                    }
                }
                Err(err_msg) => {
                    eprintln!("{}", err_msg);
                    err_msg.to_string()
                }
            };

            context
                .send_message_in_reply(&message)
                .call()
                .await
                .unwrap();
        }
    });

    bot.polling().start().await.unwrap();

    Ok(())
}
