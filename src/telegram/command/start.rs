// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult, types::ParseMode};

use crate::{
    database::{entities::chat::Chat, types::lang::Lang, Connection},
    statics::{self, GREETINGS_CHINESE, GREETINGS_ENGLISH},
    tool::macros::unwrap_or_execute,
};

pub(super) async fn start(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    if let Some(chat) = unwrap_or_execute!(db_conn.select_chat(chat_id.0).await, |e| {
        log::error!("{e}");
        return respond(());
    }) {
        bot.send_message(chat_id, match chat.lang {
            Lang::Chinese => GREETINGS_CHINESE.to_owned(),
            Lang::English => GREETINGS_ENGLISH.to_owned(),
            Lang::Bilingual => format!("{GREETINGS_CHINESE}\n{GREETINGS_ENGLISH}"),
        })
        .reply_to_message_id(message.id)
        .await?;

        return respond(());
    }

    let lang = message
        .from()
        .and_then(|f| f.language_code.as_ref().filter(|&s| s.starts_with("zh")))
        .and(Some(Lang::Chinese))
        .unwrap_or(Lang::English);

    let chat = Chat {
        id: chat_id.0,
        lang: lang.clone(),
    };

    unwrap_or_execute!(db_conn.insert_chat(&chat).await, |e| {
        log::error!("{e}");
        return respond(());
    });

    bot.send_message(chat_id, match lang {
        Lang::Bilingual => unreachable!(),
        Lang::Chinese => statics::START_MESSAGE_CHINESE,
        Lang::English => statics::START_MESSAGE_ENGLISH,
    })
    .parse_mode(ParseMode::Html)
    .reply_to_message_id(message.id)
    .await?;

    respond(())
}
