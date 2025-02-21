// Copyright (c) 2022 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{
	prelude::*,
	types::{ParseMode, ReplyParameters},
};

use super::macros::reply_html;
use crate::{
	answer::{Answer as _, Warning as Answer},
	database::entities::chat::Chat,
	statics::get_bilingual_str,
};

pub(super) async fn warning(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	let pieces = Answer::answer(&chat.lang).await;

	if pieces.is_empty() {
		bot.send_message(chat_id, get_bilingual_str!(chat.lang, NO_WARNING_MESSAGE))
			.reply_parameters(ReplyParameters::new(message.id))
			.await?;
	}

	for p in pieces {
		reply_html!(chat_id, message.id, p, bot)?;
	}

	respond(())
}
