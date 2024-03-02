// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, types::ParseMode};

use super::macros::reply_html;
use crate::{
    database::{entities::chat::Chat, types::lang::Lang, Connection},
    statics,
};

pub(super) async fn start(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    if let Some(chat) = match db_conn.select_chat(chat_id.0).await {
        Ok(chat) => chat,
        Err(e) => {
            log::error!("{e}");
            return respond(());
        }
    } {
        let text = match chat.lang {
            Lang::Chinese => statics::GREETINGS_CHINESE,
            Lang::English => statics::GREETINGS_ENGLISH,
            Lang::Bilingual => statics::GREETINGS_BILINGUAL,
        };
        reply_html!(chat_id, message.id, text, bot)?;

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

    if let Err(e) = db_conn.insert_chat(&chat).await {
        log::error!("{e}");
        return respond(());
    };

    let text = match lang {
        Lang::Bilingual => unreachable!(),
        Lang::Chinese => statics::START_MESSAGE_CHINESE,
        Lang::English => statics::START_MESSAGE_ENGLISH,
    };
    reply_html!(chat_id, message.id, text, bot)?;

    respond(())
}
