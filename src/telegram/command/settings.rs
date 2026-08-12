// Copyright (c) 2022 - 2026 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{
	prelude::*,
	types::{InlineKeyboardButton, ParseMode, ReplyMarkup},
};

use crate::{
	database::{entities::chat::Chat, types::lang::Lang},
	statics,
};

pub(super) async fn settings(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	let (msg1, settings_lang1, settings_lang, msg2) = match chat.lang {
		Lang::Bilingual => (
			statics::SETTINGS_MESSAGE_1_BILINGUAL,
			statics::SETTINGS_MESSAGE_LANGUAGE_BILINGUAL,
			"<b>語言 Language</b>\n雙語 Bilingual",
			statics::SETTINGS_MESSAGE_2_BILINGUAL,
		),
		Lang::Chinese => (
			statics::SETTINGS_MESSAGE_1_CHINESE,
			statics::SETTINGS_MESSAGE_LANGUAGE_CHINESE,
			"<b>語言</b>\n中文",
			statics::SETTINGS_MESSAGE_2_CHINESE,
		),
		Lang::English => (
			statics::SETTINGS_MESSAGE_1_ENGLISH,
			statics::SETTINGS_MESSAGE_LANGUAGE_ENGLISH,
			"<b>Language</b>\nEnglish",
			statics::SETTINGS_MESSAGE_2_ENGLISH,
		),
	};

	bot.send_message(chat_id, msg1.to_string() + "\n\n" + settings_lang + "\n\n" + msg2)
		.parse_mode(ParseMode::Html)
		.reply_markup(ReplyMarkup::inline_kb(vec![vec![InlineKeyboardButton::callback(settings_lang1, "/setlang")]]))
		.await?;

	respond(())
}
