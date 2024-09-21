// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use teloxide::{prelude::*, types::ParseMode};

use super::macros::reply_html;
use crate::{
	answer::{Answer as _, Forecast as Answer},
	database::entities::chat::Chat,
};

pub(super) async fn forecast(days: Option<usize>, message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	let answer = Answer::answer(&chat.lang).await;
	if answer.is_empty() {
		log::error!("Empty forecast data");
		return respond(());
	}

	let mut reply = answer.first().cloned().unwrap_or_default();
	for p in answer.iter().skip(1).take(days.unwrap_or(9)) {
		write!(reply, "\n\n{}", p.clone()).ok();
	}
	write!(reply, "\n\n{}", answer.last().cloned().unwrap_or_default()).ok();

	reply_html!(chat_id, message.id, reply, bot)?;

	respond(())
}
