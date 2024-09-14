// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, types::ParseMode};

use super::macros::reply_html;
use crate::{
	answer::{Answer as _, Bulletin},
	database::entities::chat::Chat,
	tool::ext::NonEmptyExt as _,
};

pub(super) async fn bulletin(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	if let Some(text) = Bulletin::answer(&chat.lang).await.get_non_empty() {
		reply_html!(chat_id, message.id, text, bot)?;
	} else {
		log::error!("Empty bulletin data");
	}

	respond(())
}
