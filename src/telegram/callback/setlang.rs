// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::str::FromStr;

use teloxide::{
    prelude::*,
    requests::ResponseResult,
    types::{InlineKeyboardMarkup, ParseMode},
};

use crate::{
    database::{types::lang::Lang, Connection},
    statics,
    telegram::{misc::start_first, setlang_ikb, setlang_internal},
};

pub(super) async fn setlang(
    lang: Option<String>,
    callback: CallbackQuery,
    bot: Bot,
    db_conn: Connection,
) -> ResponseResult<()> {
    if callback.message.is_none() {
        return respond(());
    }

    let message = callback.message.unwrap();
    let chat_id = message.chat.id;

    if lang.is_none() {
        bot.edit_message_text(chat_id, message.id, statics::SETLANG_QUESTION_BILINGUAL)
            .reply_markup(InlineKeyboardMarkup {
                inline_keyboard: setlang_ikb(),
            })
            .await?;

        return respond(());
    }

    let Ok(lang) = Lang::from_str(&lang.unwrap()) else {
        return respond(())
    };

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

    setlang_internal(lang.clone(), chat, db_conn, || async {
        bot.edit_message_text(chat_id, message.id, match lang {
            Lang::Bilingual => statics::SETLANG_MESSAGE_BILINGUAL,
            Lang::Chinese => statics::SETLANG_MESSAGE_CHINESE,
            Lang::English => statics::SETLANG_MESSAGE_ENGLISH,
        })
        .parse_mode(ParseMode::Html)
        .await?;

        respond(())
    })
    .await?;

    respond(())
}
