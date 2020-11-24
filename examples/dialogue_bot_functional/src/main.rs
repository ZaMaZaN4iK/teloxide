// This bot throws a dice on each incoming message.

use mux_stream::{demux, dispatch, error_handler};
use teloxide::prelude::*;
use futures::Stream;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();
    let updates = default_updates_stream(bot);

    let (messages,) = demux!(UpdateKind { Message }..)(updates, error_handler::panicking());
    handle_messages(messages).await;
}

async fn handle_messages(messages: impl Stream<Item = Message>) {

}
