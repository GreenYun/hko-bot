// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::prelude::*;

use crate::{
    database::{entities::chat::Chat, types::lang::Lang},
    statics,
};

pub(super) async fn help(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    bot.send_message(chat_id, match chat.lang {
        Lang::Bilingual => statics::HELP_MESSAGE_BILINGUAL,
        Lang::Chinese => statics::HELP_MESSAGE_CHINESE,
        Lang::English => statics::HELP_MESSAGE_ENGLISH,
    })
    .reply_to_message_id(message.id)
    .await?;

    respond(())
}
