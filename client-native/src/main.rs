use anyhow::Result;
use clap::Parser;
use common::message::{client, server};
use futures::{SinkExt, StreamExt};
use kodec::binary::Codec;
use mezzenger::{Messages, Receive};
use tokio_tungstenite::connect_async;
use url::Url;

/// Web app native client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server URL.
    #[arg(short, long, default_value = "ws://localhost:3030/ws")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let url = Url::parse(&args.url)?;
    let (web_socket, _) = connect_async(url).await?;

    let (web_socket_sender, web_socket_receiver) = web_socket.split();
    let codec = Codec::default();
    let mut sender =
        { mezzenger_websocket::Sender::<_, Codec, server::Message>::new(web_socket_sender, codec) };
    let mut receiver = {
        mezzenger_websocket::Receiver::<_, Codec, client::Message>::new(web_socket_receiver, codec)
    };

    

    Ok(())
}
