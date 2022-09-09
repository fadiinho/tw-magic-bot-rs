use twitch_irc::message::ServerMessage;

use super::parser::{parse_message, Command};

pub fn handle_message(message: &ServerMessage) -> Option<Command> {
    let parsed_message = parse_message(message);

    parsed_message.as_ref()?;

    let msg = parsed_message.unwrap();

    Some(msg)
}
