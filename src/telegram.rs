// Copyright (c) 2022 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::Arc;

use teloxide::{
	RequestError,
	dispatching::{DefaultKey, UpdateHandler},
	prelude::*,
};

use crate::{database::Connection, http, trigger};

pub fn connect<S>(token: S, db_conn: Connection) -> Dispatcher<Bot, RequestError, DefaultKey>
where
	S: Into<String> + Send + Sync,
{
	log::info!("Connecting to Telegram...");

	let bot = Bot::with_client(token, http::client());

	trigger::set_bot(bot.clone());

	{
		let bot = bot.clone();
		tokio::spawn(async move {
			match bot.get_me().await {
				Ok(me) => log::info!("Connected to Telegram bot {}", me.full_name()),
				Err(e) => log::error!("Connection error: {e}"),
			}
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
	use callback::schema as callback;
	use command::schema as command;
	use inlineq::schema as inlineq;

	dptree::entry()
		.branch(Update::filter_message().branch(command()))
		.branch(Update::filter_callback_query().branch(callback()))
		.branch(Update::filter_inline_query().branch(inlineq()))
}

mod callback;
mod command;
mod inlineq;
mod misc;
