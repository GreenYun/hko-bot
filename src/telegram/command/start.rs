// Copyright (c) 2022 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, types::ParseMode};

use super::macros::reply_html;
use crate::{
	database::{Connection, entities::chat::Chat, types::lang::Lang},
	statics::{self, get_bilingual_str},
};

pub(super) async fn start(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	if let Some(chat) = match db_conn.select_chat(chat_id.0).await {
		Ok(chat) => chat,
		Err(e) => {
			log::error!("{e}");
			return respond(());
		}
	} {
		let text = get_bilingual_str!(chat.lang, GREETINGS);
		reply_html!(chat_id, message.id, text, bot)?;

		return respond(());
	}

	let lang = message
		.from
		.and_then(|f| f.language_code.filter(|s| s.starts_with("zh")))
		.and(Some(Lang::Chinese))
		.unwrap_or(Lang::English);

	let chat = Chat { id: chat_id.0, lang: lang.clone() };

	if let Err(e) = db_conn.insert_chat(&chat).await {
		log::error!("{e}");
		return respond(());
	}

	let text = lang.map("", statics::START_MESSAGE_CHINESE, statics::START_MESSAGE_ENGLISH);
	reply_html!(chat_id, message.id, text, bot)?;

	respond(())
}
