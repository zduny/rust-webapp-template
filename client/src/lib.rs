use std::{cell::RefCell, rc::Rc};

use common::message::{
    client, server,
    worker::{self, host},
};
use futures::{SinkExt, StreamExt};
use kodec::binary::Codec;
use wasm_bindgen::{prelude::*, JsCast};

use js_utils::{console_log, document, event::When, set_panic_hook, spawn, window};
use mezzenger::{Messages, Receive};
use web_sys::{
    HtmlInputElement, HtmlTextAreaElement, KeyboardEvent, MouseEvent, WebSocket, Worker,
};

#[wasm_bindgen(start)]
pub async fn main_client() -> Result<(), JsValue> {
    set_panic_hook();

    console_log!("Hello World!");

    let document = document();

    let text_area = Rc::new(
        document
            .get_element_by_id("text")
            .unwrap()
            .dyn_into::<HtmlTextAreaElement>()
            .unwrap(),
    );

    let write_line = move |line: &str| {
        let mut value = text_area.value();
        value += line;
        value += "\n";
        text_area.set_value(&value);
        text_area.set_scroll_top(text_area.scroll_height());
    };

    write_line("Hello.");

    // setting up web socket
    write_line("Connecting to server...");
    let host = window()
        .location()
        .host()
        .expect("couldn't extract host from location");
    let url = format!("ws://{host}/ws");
    let web_socket = Rc::new(WebSocket::new(&url).unwrap());
    let (web_socket_sender, mut web_socket_receiver) = mezzenger_websocket::Transport::<
        Codec,
        server::Message,
        client::Message,
    >::new(&web_socket, Codec::default())
    .await
    .unwrap()
    .split();
    let web_socket_sender = Rc::new(RefCell::new(web_socket_sender));
    write_line("Connected.");

    // setting up worker
    let worker = Rc::new(Worker::new("./worker.js").unwrap());
    let (worker_sender, worker_receiver) =
        mezzenger_webworker::Transport::<_, Codec, worker::Message, host::Message>::new(
            &worker,
            Codec::default(),
        )
        .await
        .unwrap()
        .split();
    let worker_sender = Rc::new(RefCell::new(worker_sender));

    // handle worker messages
    let write_line_clone = write_line.clone();
    let mut message_stream = worker_receiver.messages_with_error_callback(move |error| {
        write_line_clone(&format!(
            "Error occurred while receiving message from worker: {error}."
        ));
    });
    let write_line_clone = write_line.clone();
    spawn(async move {
        while let Some(message) = message_stream.next().await {
            match message {
                worker::Message::ResultFactorial { input, output } => {
                    write_line_clone(&format!("{input}! = {output}"));
                }
                worker::Message::ResultFibonacci { input, output } => {
                    write_line_clone(&format!("fibonacci({input}) = {output}"));
                }
            }
        }
    });

    // setting up event handlers
    let input = Rc::new(
        document
            .get_element_by_id("input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap(),
    );

    let number = Rc::new(
        document
            .get_element_by_id("number")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap(),
    );

    let input_clone = input.clone();
    let write_line_clone = write_line.clone();
    let send = move || {
        let text = input_clone.value().trim().to_string();
        let web_socket_sender = web_socket_sender.clone();
        let write_line_clone = write_line_clone.clone();
        spawn(async move {
            let mut sender = web_socket_sender.borrow_mut();
            match sender.send(client::Message { content: text }).await {
                Ok(_) => (),
                Err(error) => {
                    write_line_clone(&format!("Error occurred while sending message: {error}."));
                }
            };
        });
        input_clone.set_value("");
    };

    let send_clone = send.clone();
    let _input_handler = input
        .when("keyup", move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                send_clone();
            }
        })
        .unwrap();

    let _send_handler = Rc::new(document.get_element_by_id("send").unwrap())
        .when("click", move |_event: MouseEvent| {
            send();
        })
        .unwrap();

    let number_clone = number.clone();
    let write_line_clone = write_line.clone();
    let get_input = move || {
        let value = number_clone.value();
        let input = value.parse::<u64>();
        if input.is_err() {
            write_line_clone(&format!("Error: {value} is not a non-negative integer."));
        }
        input.ok()
    };

    let get_input_clone = get_input.clone();
    let write_line_clone = write_line.clone();
    let worker_sender_clone = worker_sender.clone();
    let _fibonacci_handler = Rc::new(document.get_element_by_id("fibonacci").unwrap())
        .when("click", move |_event: MouseEvent| {
            if let Some(input) = get_input_clone() {
                write_line_clone(&format!("Calculating fibonacci({input})..."));
                let worker_sender_clone = worker_sender_clone.clone();
                let write_line_clone = write_line_clone.clone();
                spawn(async move {
                    if let Err(error) = worker_sender_clone
                        .borrow_mut()
                        .send(host::Message::CalculateFibonacci { input })
                        .await
                    {
                        write_line_clone(&format!(
                            "Error occurred while sending message to worker: {error}."
                        ));
                    }
                });
            }
        })
        .unwrap();

    let write_line_clone = write_line.clone();
    let _factorial_handler = Rc::new(document.get_element_by_id("factorial").unwrap())
        .when("click", move |_event: MouseEvent| {
            if let Some(input) = get_input() {
                let worker_sender_clone = worker_sender.clone();
                let write_line_clone = write_line_clone.clone();
                write_line_clone(&format!("Calculating {input}!..."));
                spawn(async move {
                    if let Err(error) = worker_sender_clone
                        .borrow_mut()
                        .send(host::Message::CalculateFactorial { input })
                        .await
                    {
                        write_line_clone(&format!(
                            "Error occurred while sending message to worker: {error}."
                        ));
                    }
                });
            }
        })
        .unwrap();

    // handle server messages
    let init_message = web_socket_receiver.receive().await.unwrap();
    match init_message {
        server::Message::Init {
            user_name,
            connected_user_names,
        } => {
            write_line(&format!("Your name: <{user_name}>."));

            if !connected_user_names.is_empty() {
                write_line(&format!(
                    "Other connected users: {}.",
                    connected_user_names
                        .iter()
                        .map(|user| format!("<{user}>"))
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }
        }
        _ => panic!("unexpected message received"),
    }

    let write_line_clone = write_line.clone();
    let mut message_stream = web_socket_receiver.messages_with_error_callback(move |error| {
        write_line_clone(&format!(
            "Error occurred while receiving message from web socket: {error}."
        ));
    });

    while let Some(message) = message_stream.next().await {
        match message {
            server::Message::UserConnected { user_name } => {
                write_line(&format!("New user connected: <{user_name}>."));
            }
            server::Message::UserDisconnected { user_name } => {
                write_line(&format!("User <{user_name}> left."));
            }
            server::Message::Message { user_name, content } => {
                write_line(&format!("<{user_name}> {content}"));
            }
            _ => panic!("unexpected message received"),
        }
    }
    write_line("Server disconnected.");

    Ok(())
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::add_numbers;

    #[wasm_bindgen_test]
    fn test_add_numbers() {
        assert_eq!(5, add_numbers(2, 3));
    }
}
