// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use teloxide::{
    prelude::*,
    types::{ParseMode, ReplyMarkup},
};

use super::macros::reply_html;
use crate::{
    answer,
    database::{entities::chat::Chat, types::lang::Lang, Connection},
    statics,
    telegram::misc::{setlang_ikb, setlang_internal},
};

pub(super) async fn setlang(
    lang: Option<String>,
    message: Message,
    bot: Bot,
    chat: Chat,
    db_conn: Connection,
) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    let Some(lang) = lang.and_then(|lang| Lang::from_str(&lang).ok()) else {
        return setlang_question(message, bot.clone()).await;
    };

    if setlang_internal(&lang, chat, db_conn).await {
        reply_html!(chat_id, message.id, answer::setlang(&lang), bot)?;
    }

    respond(())
}

async fn setlang_question(message: Message, bot: Bot) -> ResponseResult<()> {
    bot.send_message(message.chat.id, statics::SETLANG_QUESTION_BILINGUAL)
        .reply_markup(ReplyMarkup::inline_kb(setlang_ikb()))
        .reply_to_message_id(message.id)
        .await?;

    respond(())
}
