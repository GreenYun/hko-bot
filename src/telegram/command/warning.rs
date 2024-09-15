// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use teloxide::{
	prelude::*,
	types::{ParseMode, ReplyParameters},
	ApiError, RequestError,
};

use super::macros::reply_html;
use crate::{
	database::{entities::chat::Chat, types::lang::Lang},
	statics::get_bilingual_str,
	tool::mix_strings,
	weather::{Warning as Data, WeatherData},
};

pub(super) async fn warning(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	let Some(warning) = Data::get().await else {
		bot.send_message(chat_id, "Connection timed out, please try again later.")
			.reply_parameters(ReplyParameters::new(message.id))
			.await?;
		return respond(());
	};

	if warning.pieces.is_empty() {
		bot.send_message(chat_id, get_bilingual_str!(chat.lang, NO_WARNING_MESSAGE))
			.reply_parameters(ReplyParameters::new(message.id))
			.await?;
	}

	for w in warning.pieces {
		let mut list = vec!["<b>".to_string() + w.name + "</b>"];
		list.extend_from_slice(&w.contents);

		let mut text = mix_strings(&chat.lang, &list);
		write!(text, "\n\n<i>@ {}</i>", w.update_time).ok();

		let send = reply_html!(chat_id, message.id, text, bot);

		match send {
			Ok(_) => {}
			Err(RequestError::Api(ApiError::MessageIsTooLong)) => {
				if matches!(chat.lang, Lang::Bilingual) {
					text = mix_strings(&Lang::Chinese, &list);
					write!(text, "\n\n<i>@ {}</i>", w.update_time).ok();
					reply_html!(chat_id, message.id, text, bot)?;

					text = mix_strings(&Lang::English, &list);
					write!(text, "\n\n<i>@ {}</i>", w.update_time).ok();
					reply_html!(chat_id, message.id, text, bot)?;
				}
			}
			Err(e) => {
				return Err(e);
			}
		}
	}

	respond(())
}
