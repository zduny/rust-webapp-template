use common::{
    factorial, fibonacci,
    message::worker::{self, host},
};
use futures::{SinkExt, StreamExt};
use js_utils::{console_log, set_panic_hook};
use kodec::binary::Codec;
use mezzenger::Messages;
use mezzenger_webworker::Transport;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_worker() -> Result<(), JsValue> {
    set_panic_hook();

    console_log!("Worker: worker started!");
    let (mut sender, receiver) =
        Transport::<_, Codec, host::Message, worker::Message>::new_in_worker(Codec::default())
            .await
            .unwrap()
            .split();
    console_log!("Worker: transport open.");

    let mut message_stream = receiver.messages_with_error_callback(move |error| {
        console_log!("Worker: error occurred while receiving message from host: {error}.");
    });
    while let Some(message) = message_stream.next().await {
        match message {
            host::Message::CalculateFactorial { input } => {
                console_log!("Worker: calculating {input}!...");
                let output = factorial(input);
                if let Err(error) = sender
                    .send(worker::Message::ResultFactorial { input, output })
                    .await
                {
                    console_log!("Worker: error occurred while sending message: {error}");
                }
            }
            host::Message::CalculateFibonacci { input } => {
                console_log!("Worker: calculating fibonacci({input})...");
                let output = fibonacci(input);
                if let Err(error) = sender
                    .send(worker::Message::ResultFibonacci { input, output })
                    .await
                {
                    console_log!("Worker: error occurred while sending message: {error}");
                }
            }
        }
    }

    Ok(())
}
