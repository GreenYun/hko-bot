// Copyright (c) 2022 - 2025 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{prelude::*, types::InlineKeyboardButton};

use crate::database::{Connection, entities::chat::Chat, types::lang::Lang};

pub async fn start_first(bot: Bot, chat_id: ChatId) -> ResponseResult<()> {
	bot.send_message(chat_id, "/start first.").await?;

	respond(())
}

pub fn setlang_ikb() -> Vec<Vec<InlineKeyboardButton>> {
	vec![vec![
		InlineKeyboardButton::callback("雙語\nBilingual", "/setlang bilingual"),
		InlineKeyboardButton::callback("中文", "/setlang chinese"),
		InlineKeyboardButton::callback("English", "/setlang english"),
	]]
}

pub async fn setlang_internal(lang: &Lang, chat: Chat, db_conn: Connection) -> bool {
	if lang == &chat.lang {
		return true;
	}

	let mut chat = chat.clone();
	chat.lang = lang.clone();

	match db_conn.update_chat(&chat).await {
		Ok(res) => res.rows_affected() > 0,
		Err(e) => {
			log::error!("{e}");
			false
		}
	}
}
