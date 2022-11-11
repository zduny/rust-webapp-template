mod state;

use std::{env::current_dir, sync::Arc};

use anyhow::Result;
use common::message::{client, server};
use futures::{SinkExt, StreamExt};
use kodec::binary::Codec;
use mezzenger_websocket::warp::{Receiver, Sender};
use tokio::{
    signal::ctrl_c,
    spawn,
    sync::{mpsc::unbounded_channel, RwLock},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{error, info, Level};
use warp::{
    hyper::StatusCode,
    ws::{WebSocket, Ws},
    Filter,
};

use crate::state::User;

type State = Arc<RwLock<state::State>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Server running!");

    let current_dir = current_dir()?;
    info!("Current working directory: {:?}.", current_dir);

    let state = State::default();
    let state = warp::any().map(move || state.clone());
    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(state)
        .map(|ws: Ws, state| ws.on_upgrade(move |web_socket| user_connected(web_socket, state)));

    let static_files = warp::get().and(warp::fs::dir("www"));
    let routes = websocket.or(static_files).recover(handle_rejection);

    let (address, server_future) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async move {
            ctrl_c()
                .await
                .expect("unable to listen for shutdown signal");
        });
    let server_handle = spawn(server_future);
    info!("Listening at {}...", address);

    server_handle.await?;
    info!("Shutting down...");

    Ok(())
}

async fn user_connected(web_socket: WebSocket, state: State) {
    let (web_socket_sender, web_socket_receiver) = web_socket.split();

    let codec = Codec::default();
    let mut sender = Sender::<_, Codec, server::Message>::new(web_socket_sender, codec);
    let mut receiver = Receiver::<_, Codec, client::Message>::new(web_socket_receiver, codec);

    let (user_sender, user_receiver) = unbounded_channel();
    let mut user_receiver = UnboundedReceiverStream::new(user_receiver);

    let user = User::new(user_sender);
    let id = user.id;
    let name = user.name.clone();

    let connected_user_names = state
        .read()
        .await
        .users
        .values()
        .map(|user| user.name.clone())
        .collect();
    let init_message = server::Message::Init {
        user_name: name.clone(),
        connected_user_names,
    };
    if sender.send(init_message).await.is_ok() {
        info!("User <{name}> connected.");
        let message = server::Message::UserConnected {
            user_name: name.clone(),
        };
        for (&user_id, user) in state.read().await.users.iter() {
            if id != user_id {
                let _ = user.sender.send(message.clone());
            }
        }

        spawn(async move {
            while let Some(message) = user_receiver.next().await {
                let result = sender.send(message).await;
                if let Err(error) = result {
                    error!("Failed to send message to user: id: {id}, error: {error}.");
                }
            }
        });

        state.write().await.users.insert(id, user);

        while let Some(result) = receiver.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(error) => {
                    error!("Failed to receive message from user: id: {id}, error: {error}.");
                    break;
                }
            };
            user_message(id, name.clone(), msg, &state).await;
        }

        user_disconnected(id, name, &state).await;
    }
}

async fn user_message(_id: usize, name: String, message: client::Message, state: &State) {
    let message = server::Message::Message {
        user_name: name,
        content: message.content,
    };
    for user in state.read().await.users.values() {
        let _ = user.sender.send(message.clone());
    }
}

async fn user_disconnected(id: usize, name: String, state: &State) {
    info!("User <{name}> disconnected.");

    let message = server::Message::UserDisconnected { user_name: name };
    for (&user_id, user) in state.read().await.users.iter() {
        if id != user_id {
            let _ = user.sender.send(message.clone());
        }
    }

    state.write().await.users.remove(&id);
}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.is_not_found() {
        error!("Error occurred: {:?}.", err);
        Ok(warp::reply::with_status("Not found", StatusCode::NOT_FOUND))
    } else {
        error!("Error occurred: {:?}.", err);
        Ok(warp::reply::with_status(
            "Internal server error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
