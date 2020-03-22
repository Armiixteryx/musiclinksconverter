mod services;

use tbot::prelude::*;
use tbot::types::chat::Action;
use std::error::Error;

use services::{deezer::{DeezerController, DEEZER_DOMAIN}, spotify::{SpotifyController, SPOTIFY_DOMAIN}, Services};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {

            let user_msg = &context.text.value;

            let service = {
                if user_msg.starts_with(DEEZER_DOMAIN) {
                    Services::Deezer
                }
                else if user_msg.starts_with(SPOTIFY_DOMAIN) {
                    Services::Spotify
                } else {
                    Services::Unsupported
                }
            };

            dbg!(&service);

            let control_deezer = DeezerController::new();
            let control_spotify = SpotifyController::new().await.unwrap();

            // `Typing...` status in chat
            context.send_chat_action(Action::Typing).call().await.unwrap();

            let message = match service {
                Services::Deezer => {
                    let deezer_data = control_deezer.analyze_url(&context.text.value).await.unwrap();

                    control_spotify.generate_url(&deezer_data).await.unwrap()
                },
                Services::Spotify => {
                    let spotify_data = control_spotify.analyze_url(&context.text.value).await.unwrap_or_else(|e| {
                        eprintln!("Error: {}", e);
                        eprintln!("Caused by: {}", e.source().unwrap());
                        panic!();
                    });

                    control_deezer.generate_url(&spotify_data).await.unwrap()
                },
                Services::Unsupported => "Invalid link. Please send other link.".to_string()
            };
            
            context.send_message_in_reply(&message).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();

    Ok(())
}
