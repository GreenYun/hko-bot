// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{future::Future, str::FromStr};

use teloxide::{
    prelude::*,
    requests::ResponseResult,
    types::{InlineKeyboardButton, ParseMode, ReplyMarkup},
};

use crate::{
    database::{entities::chat::Chat, types::lang::Lang, Connection},
    statics,
    telegram::misc::start_first,
};

pub(super) async fn setlang(
    lang: Option<String>,
    message: Message,
    bot: Bot,
    db_conn: Connection,
) -> ResponseResult<()> {
    let chat_id = message.chat.id;
    let chat = match db_conn.select_chat(chat_id.0).await {
        Ok(chat) => {
            let Some(chat) = chat else {
                return start_first(bot, chat_id).await;
            };

            chat
        }
        Err(e) => {
            log::error!("{e}");
            return respond(());
        }
    };

    let Some(lang) = lang.and_then(|lang| Lang::from_str(&lang).ok()) else {
        return setlang_question(message, bot.clone()).await;
    };

    setlang_internal(lang.clone(), chat, db_conn, || async {
        bot.send_message(chat_id, match lang {
            Lang::Bilingual => statics::SETLANG_MESSAGE_BILINGUAL,
            Lang::Chinese => statics::SETLANG_MESSAGE_CHINESE,
            Lang::English => statics::SETLANG_MESSAGE_ENGLISH,
        })
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(message.id)
        .await?;

        respond(())
    })
    .await?;

    respond(())
}

pub(in super::super) async fn setlang_internal<F, Fut>(
    lang: Lang,
    chat: Chat,
    db_conn: Connection,
    f: F,
) -> ResponseResult<()>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = ResponseResult<()>> + Send,
{
    let success = lang == chat.lang || {
        let mut chat = chat.clone();
        chat.lang = lang.clone();

        match db_conn.update_chat(&chat).await {
            Ok(res) => res.rows_affected() > 0,
            Err(e) => {
                log::error!("{e}");
                return respond(());
            }
        }
    };

    if success {
        f().await?;
    }

    respond(())
}

pub(in super::super) fn setlang_ikb() -> Vec<Vec<InlineKeyboardButton>> {
    vec![vec![
        InlineKeyboardButton::callback("雙語\nBilingual", "/setlang bilingual"),
        InlineKeyboardButton::callback("中文", "/setlang chinese"),
        InlineKeyboardButton::callback("English", "/setlang english"),
    ]]
}

async fn setlang_question(message: Message, bot: Bot) -> ResponseResult<()> {
    bot.send_message(message.chat.id, statics::SETLANG_QUESTION_BILINGUAL)
        .reply_markup(ReplyMarkup::inline_kb(setlang_ikb()))
        .reply_to_message_id(message.id)
        .await?;

    respond(())
}
