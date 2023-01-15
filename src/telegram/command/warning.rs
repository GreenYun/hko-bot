// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use teloxide::{prelude::*, requests::ResponseResult, types::ParseMode};

use crate::{
    database::{types::lang::Lang, Connection},
    statics,
    telegram::misc::start_first,
    tool::{mix_strings, try_data},
    weather,
};

pub(super) async fn warning(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
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

    let Some(warning) = try_data(weather::warning, |_| true).await else {
        bot.send_message(chat_id, "Connection timed out, please try again later.")
            .reply_to_message_id(message.id)
            .await?;
        return respond(());
    };

    if warning.pieces.is_empty() {
        bot.send_message(chat_id, match &chat.lang {
            Lang::Bilingual => statics::NO_WARNING_MESSAGE_BILINGUAL,
            Lang::Chinese => statics::NO_WARNING_MESSAGE_CHINESE,
            Lang::English => statics::NO_WARNING_MESSAGE_ENGLISH,
        })
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(message.id)
        .await?;
    }

    for w in warning.pieces {
        let mut list = vec!["<b>".to_owned() + w.name + "</b>"];
        list.extend_from_slice(&w.contents);

        let mut text = mix_strings(list, &chat.lang);
        write!(text, "<i>@ {}</i>", w.update_time).ok();

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .reply_to_message_id(message.id)
            .await?;
    }

    respond(())
}
