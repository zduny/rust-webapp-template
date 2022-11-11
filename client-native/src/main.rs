use anyhow::Result;
use clap::Parser;
use common::message::{client, server};
use futures::{select, FutureExt, SinkExt, StreamExt};
use kodec::binary::Codec;
use lazy_static::lazy_static;
use mezzenger::{Messages, Receive};
use regex::{Captures, Regex};
use rustyline_async::{Readline, ReadlineError, SharedWriter};
use std::io::Write;
use tokio::spawn;
use tokio_tungstenite::connect_async;
use url::Url;

lazy_static! {
    static ref FIBONACCI_PATTERN: Regex = Regex::new(r"^fibonacci\((0|[1-9][0-9]*)\)$").unwrap();
    static ref FACTORIAL_PATTERN: Regex = Regex::new(r"^(0|[1-9][0-9]*)!$").unwrap();
}
/// Web app native client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server URL.
    #[arg(short, long, default_value = "ws://localhost:8080/ws")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Hello.");

    println!("Connecting to server...");
    let url = Url::parse(&args.url)?;
    let (web_socket, _) = connect_async(url).await?;

    let (web_socket_sender, web_socket_receiver) = web_socket.split();
    let codec = Codec::default();
    let mut sender =
        { mezzenger_websocket::Sender::<_, Codec, client::Message>::new(web_socket_sender, codec) };
    let mut receiver = {
        mezzenger_websocket::Receiver::<_, Codec, server::Message>::new(web_socket_receiver, codec)
    };
    println!("Connected.");

    let init_message = receiver.receive().await?;
    match init_message {
        server::Message::Init {
            user_name,
            connected_user_names,
        } => {
            println!("Your name: <{user_name}>.");

            if !connected_user_names.is_empty() {
                println!("Connected users:");
                for user_name in connected_user_names.iter() {
                    println!("<{user_name}>");
                }
            }
        }
        _ => panic!("unexpected message received"),
    }

    let (mut readline, mut stdout) = Readline::new("> ".to_string())?;

    writeln!(
        stdout,
        "Type 'fibonacci(n)' to calculate n-th element of Fibonacci sequence."
    )?;
    writeln!(stdout, "Type 'n!' to calculate factorial on n.")?;

    {
        let mut stdout_clone = stdout.clone();
        let mut message_stream = receiver.messages_with_error_callback(move |error| {
            let _ = writeln!(
                stdout_clone,
                "Error occurred while receiving message: {error}."
            );
        });

        loop {
            select! {
                message = message_stream.next() => {
                    if let Some(message) = message {
                        match message {
                            server::Message::UserConnected { user_name } => {
                                writeln!(stdout, "New user connected: <{user_name}>.")?;
                            },
                            server::Message::UserDisconnected { user_name } => {
                                writeln!(stdout, "User <{user_name}> left.")?;
                            },
                            server::Message::Message { user_name, content } => {
                                writeln!(stdout, "<{user_name}> {content}")?;
                            },
                            _ => panic!("unexpected message received"),
                        }
                    } else {
                        writeln!(stdout, "Server disconnected.")?;
                        writeln!(stdout, "Exiting...")?;
                        break;
                    }
                },
                command = readline.readline().fuse() => match command {
                    Ok(line) => {
                        let line = line.trim();
                        if let Some(captures) = FIBONACCI_PATTERN.captures(line) {
                            if let Some(number) = extract_number(&captures) {
                                let stdout_clone = stdout.clone();
                                spawn(async move { handle_fibonacci(stdout_clone, number).await.unwrap() });
                                readline.add_history_entry(line.to_string());
                            } else {
                                let input = captures.get(1).unwrap().as_str();
                                writeln!(stdout, "Error: {input} is not a non-negative integer.")?;
                            }
                        } else if let Some(captures) = FACTORIAL_PATTERN.captures(line) {
                            if let Some(number) = extract_number(&captures) {
                                let stdout_clone = stdout.clone();
                                spawn(async move { handle_factorial(stdout_clone, number).await.unwrap(); });
                                readline.add_history_entry(line.to_string());
                            } else {
                                let input = captures.get(1).unwrap().as_str();
                                writeln!(stdout, "Error: {input} is not a non-negative integer.")?;
                            }
                        } else {
                            let message = client::Message { content: line.to_string() };
                            sender.send(message).await?;
                        }
                    },
                    Err(ReadlineError::Eof | ReadlineError::Interrupted) => {
                        writeln!(stdout, "Exiting...")?;
                        break;
                    },
                    Err(error) => {
                        writeln!(stdout, "Error occurred while handling command: {error}")?;
                        writeln!(stdout, "Exiting...")?;
                        break;
                    },
                },
            }
        }
    }

    drop(stdout);
    let _ = readline.readline().await;

    Ok(())
}

fn extract_number<'a>(captures: &'a Captures<'a>) -> Option<u64> {
    captures
        .get(1)
        .and_then(|capture| capture.as_str().parse::<u64>().ok())
}

async fn handle_fibonacci(mut stdout: SharedWriter, number: u64) -> Result<()> {
    writeln!(stdout, "Calculating fibonacci({number})...")?;
    let result = tokio_rayon::spawn(move || common::fibonacci(number)).await;
    writeln!(stdout, "fibonacci({number}) = {result}")?;
    Ok(())
}

async fn handle_factorial(mut stdout: SharedWriter, number: u64) -> Result<()> {
    writeln!(stdout, "Calculating {number}!...")?;
    let result = tokio_rayon::spawn(move || common::factorial(number)).await;
    writeln!(stdout, "{number}! = {result}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{extract_number, FACTORIAL_PATTERN, FIBONACCI_PATTERN};

    #[test]
    fn test_fibonacci_pattern() {
        let captures = FIBONACCI_PATTERN.captures("fibonacci(123)").unwrap();
        let number = extract_number(&captures).unwrap();
        assert_eq!(number, 123);
    }

    #[test]
    fn test_factorial_pattern() {
        let captures = FACTORIAL_PATTERN.captures("0!").unwrap();
        let number = extract_number(&captures).unwrap();
        assert_eq!(number, 0);
    }
}
