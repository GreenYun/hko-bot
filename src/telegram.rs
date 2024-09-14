// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::Arc;

use teloxide::{
	dispatching::{DefaultKey, UpdateHandler},
	prelude::*,
	RequestError,
};

use crate::{database::Connection, http};

pub fn connect<S>(token: S, db_conn: Connection) -> Dispatcher<Bot, RequestError, DefaultKey>
where
	S: Into<String> + Send + Sync,
{
	log::info!("Connecting to Telegram...");

	let bot = Bot::with_client(token, http::client());

	{
		let bot = bot.clone();
		tokio::spawn(async move {
			match bot.get_me().await {
				Ok(me) => log::info!("Connected to Telegram bot {}", me.full_name()),
				Err(e) => log::error!("Connection error: {e}"),
			};
		});
	}

	let mut dependencies = DependencyMap::new();
	dependencies.insert(db_conn);

	let error_handler = |e| async move {
		log::error!("{e}");
	};

	Dispatcher::builder(bot, schema())
		.dependencies(dependencies)
		.default_handler(|update| async move { log::debug!("{update:?}") })
		.error_handler(Arc::new(error_handler))
		.enable_ctrlc_handler()
		.build()
}

fn schema() -> UpdateHandler<RequestError> {
	let callback_handler = Update::filter_callback_query().branch(callback::schema());
	let inline_query_handler = Update::filter_inline_query().branch(inlineq::schema());
	let message_handler = Update::filter_message().branch(command::schema());

	dptree::entry().branch(message_handler).branch(callback_handler).branch(inline_query_handler)
}

mod callback;
mod command;
mod inlineq;
mod misc;
