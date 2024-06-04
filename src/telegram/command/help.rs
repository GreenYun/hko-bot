// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::prelude::*;

use crate::{database::entities::chat::Chat, statics::get_bilingual_str};

pub(super) async fn help(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    bot.send_message(chat_id, get_bilingual_str!(chat.lang, HELP_MESSAGE))
        .reply_to_message_id(message.id)
        .await?;

    respond(())
}
