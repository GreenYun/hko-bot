// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use teloxide::{
	prelude::*,
	types::{InlineKeyboardMarkup, ParseMode},
};

use crate::{
	answer,
	database::{types::lang::Lang, Connection},
	statics,
	telegram::misc::{setlang_ikb, setlang_internal, start_first},
};

pub(super) async fn setlang(
	lang: Option<String>,
	callback: CallbackQuery,
	bot: Bot,
	db_conn: Connection,
) -> ResponseResult<()> {
	if callback.message.is_none() {
		return respond(());
	}

	let message = callback.message.unwrap();
	let chat_id = message.chat().id;

	if lang.is_none() {
		bot.edit_message_text(chat_id, message.id(), statics::SETLANG_QUESTION_BILINGUAL)
			.reply_markup(InlineKeyboardMarkup { inline_keyboard: setlang_ikb() })
			.await?;

		return respond(());
	}

	let Ok(lang) = Lang::from_str(&lang.unwrap()) else {
		return respond(());
	};

	let chat = match db_conn.select_chat(chat_id.0).await {
		Ok(chat) => {
			let Some(chat) = chat else {
				return start_first(bot, chat_id).await;
			};

			chat
		}
		Err(e) => {
			log::error!("{e}");
			return respond(());
		}
	};

	if setlang_internal(&lang, chat, db_conn).await {
		bot.edit_message_text(chat_id, message.id(), answer::setlang(&lang)).parse_mode(ParseMode::Html).await?;
	}

	respond(())
}
