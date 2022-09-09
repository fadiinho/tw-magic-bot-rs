mod handler;

use dotenv::dotenv;
use std::env;


use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use handler::command::handle_message;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let access_token: String = env::var("ACCESS_TOKEN").unwrap_or_else(|x| panic!("\nACCESS_TOKEN must be valid.\nError: {}", x));
    let bot_username: String = env::var("USERNAME").unwrap_or_else(|x| panic!("\nUSERNAME variable must be valid.\nError: {}", x));
    let credentials = StaticLoginCredentials::new(bot_username.to_owned(), Some(access_token.to_owned()));
    let config = ClientConfig::new_simple(credentials);

    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join("fadiinho".to_owned()).unwrap();
    client.say("fadiinho".to_owned(), "Hello, I just joined the chat!".to_owned()).await.unwrap();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await { 
            let handled_message = handle_message(&message);

            if handled_message.is_none() {
                continue;
            }

            let unwraped_message = handled_message.unwrap();
            println!("{}", unwraped_message);
            client.say("fadiinho".to_owned(), format!("{}", unwraped_message)).await.unwrap();

            println!("{:?}", message)
        }
    });

    join_handle.await.unwrap();
}
