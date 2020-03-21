mod services;

use tbot::prelude::*;
use tbot::types::chat::Action;
use std::error::Error;

use services::{deezer::DeezerController, spotify::SpotifyController};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.text(|context| {
        async move {

            // `Typing...` status in chat
            context.send_chat_action(Action::Typing).call().await.unwrap();

            let control_deezer = DeezerController::new();
            let control_spotify = SpotifyController::new().await.unwrap();

            let spotify_data = control_spotify.analyze_url(&context.text.value).await.unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                eprintln!("Caused by: {}", e.source().unwrap());
                panic!();
            });

            let message = control_deezer.generate_url(&spotify_data).await.unwrap();
            
            context.send_message_in_reply(&message).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();

    Ok(())
}
