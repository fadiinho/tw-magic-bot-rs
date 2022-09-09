use std::fmt;
use twitch_irc::message::{PrivmsgMessage, ServerMessage};

pub struct Command {
    name: String,
    args: Option<Vec<String>>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self.args.to_owned();

        write!(f, "{} ", self.name)?;

        if let Some(..) = args {
            args.unwrap().iter().for_each(|arg| {
                match write!(f, "{} ", arg) {
                    Ok(_arg) => _arg,
                    Err(error) => panic!("Something went wrong while iterating args. {:?}", error),
                };
            });
        }

        Ok(())
    }
}

fn parse_command(message: &PrivmsgMessage) -> Option<Command> {
    let message_iterable = message.message_text.split_whitespace();
    let mut command: &str = "";
    let mut args: Vec<String> = Vec::new();

    message_iterable.for_each(|arg| {
        if arg.starts_with('!') {
            command = arg;
        } else {
            args.push(arg.to_owned());
        }
    });

    if command.is_empty() {
        return None;
    }

    Some(Command {
        name: command.to_owned(),
        args: Some(args),
    })
}

pub fn parse_message(message: &ServerMessage) -> Option<Command> {
    match message {
        ServerMessage::Privmsg(msg) => parse_command(msg),
        _ => None,
    }
}
