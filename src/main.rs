mod services;

use std::error::Error;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting bot");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        log::info!("Received msg: ");
        let mut response = None;

        if let Some(txt) = message.text() {
            match services::get_service(txt) {
                Ok(url) => {
                    log::info!("{txt}");
                    match url.service {
                        services::Services::Deezer => {
                            response = Some("You have sent a Deezer link!");
                        }
                        services::Services::Spotify => {
                            response = Some("You have sent a Spotify link!");
                        }
                    }
                }
                Err(_err) => {
                    log::info!("Not a music link")
                }
            }
        }

        //log::info!("Sending dice...");

        //bot.send_dice(message.chat.id).await?;
        let response = response.unwrap_or("Not a valid message, try again");
        bot.send_message(message.chat.id, response).await?;

        respond(())
    })
    .await;

    Ok(())
}
