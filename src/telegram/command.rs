// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{
	dispatching::UpdateHandler,
	prelude::*,
	types::Me,
	utils::command::{BotCommands, ParseError},
	RequestError,
};

use super::misc::start_first;
use crate::database::Connection;
use macros::command_endpoint;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
	Briefing,
	Bulletin,
	Current,
	Help,
	Purge,

	#[command(parse_with = parse_setlang)]
	SetLang(Option<String>),
	Settings,
	Start,
	Warning,
}

#[allow(clippy::unnecessary_wraps)]
fn parse_setlang(input: String) -> Result<(Option<String>,), ParseError> {
	let input = (!input.is_empty()).then_some(input);
	Ok((input,))
}

pub fn schema() -> UpdateHandler<RequestError> {
	dptree::entry().branch(
		dptree::filter_map(move |message: Message, me: Me| {
			let bot_name = me.user.username.unwrap_or_default();
			message.text().and_then(|text| Command::parse(text, &bot_name).ok())
		})
		.branch(command_endpoint!(Command::Start))
		.branch(
			dptree::filter_map_async(|message: Message, db_conn: Connection| async move {
				let chat_id = message.chat.id;
				db_conn.select_chat(chat_id.0).await.ok().flatten()
			})
			.branch(command_endpoint!(Command::Help))
			.branch(command_endpoint!(Command::Settings))
			.branch(command_endpoint!(Command::Purge))
			.branch(command_endpoint!(Command::SetLang(lang)))
			.branch(command_endpoint!(Command::Briefing))
			.branch(command_endpoint!(Command::Bulletin))
			.branch(command_endpoint!(Command::Warning)),
		)
		.branch(dptree::endpoint(|message: Message, bot: Bot| async move {
			let chat_id = message.chat.id;
			start_first(bot, chat_id).await
		})),
	)
}

mod briefing;
mod bulletin;
mod help;
mod purge;
mod setlang;
mod settings;
mod start;
mod warning;

mod macros;
