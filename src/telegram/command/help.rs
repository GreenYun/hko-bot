// Copyright (c) 2022 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, types::ReplyParameters};

use crate::{database::entities::chat::Chat, statics::get_bilingual_str};

pub(super) async fn help(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
	let chat_id = message.chat.id;

	bot.send_message(chat_id, get_bilingual_str!(chat.lang, HELP_MESSAGE)).reply_parameters(ReplyParameters::new(message.id)).await?;

	respond(())
}
