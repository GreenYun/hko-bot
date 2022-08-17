// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::sync::Arc;

use teloxide::{
    dispatching::{self, UpdateHandler},
    prelude::*,
    RequestError,
};

use crate::database::Connection;

fn build_with_token<S>(
    token: S,
    db_conn: Connection,
) -> (
    AutoSend<Bot>,
    Dispatcher<AutoSend<Bot>, RequestError, teloxide::dispatching::DefaultKey>,
)
where
    S: Into<String>,
{
    let bot = Bot::new(token).auto_send();
    let dispatcher = build(bot.clone(), db_conn);
    (bot, dispatcher)
}

fn build(
    bot: AutoSend<Bot>,
    db_conn: Connection,
) -> Dispatcher<AutoSend<Bot>, RequestError, teloxide::dispatching::DefaultKey> {
    let mut dependencies = DependencyMap::new();
    dependencies.insert(db_conn);

    Dispatcher::builder(bot, schema())
        .dependencies(dependencies)
        .default_handler(|_| async {})
        .error_handler(Arc::new(|e| async move {
            log::error!("{:?}", e);
        }))
        .enable_ctrlc_handler()
        .build()
}

fn schema() -> UpdateHandler<RequestError> {
    let message_handler = Update::filter_message()
        .branch(command::schema())
        .branch(
            Message::filter_text().endpoint(|m: Message, b: AutoSend<Bot>| async move {
                b.send_message(m.chat.id, "Hi").await?;
                respond(())
            }),
        );

    let callback_handler = Update::filter_callback_query().branch(callback::schema());

    dptree::entry().branch(message_handler).branch(callback_handler)
}

pub async fn connect<S>(
    token: S,
    db_conn: Connection,
) -> Dispatcher<AutoSend<Bot>, RequestError, dispatching::DefaultKey>
where
    S: Into<String>,
{
    log::info!("Connecting to Telegram...");

    let (bot, dispatcher) = build_with_token(token, db_conn);

    match bot.get_me().await {
        Ok(me) => log::info!("Connected to Telegram bot {}", me.full_name()),
        Err(e) => log::error!("Connection error: {}", e),
    };

    dispatcher
}

pub(self) use command::{setlang_ikb, setlang_internal};

mod callback;
mod command;
mod misc;
