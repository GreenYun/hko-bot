// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult};

use crate::{
    database::{types::lang::Lang, Connection},
    statics,
    telegram::misc::start_first,
};

pub(super) async fn help(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
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

    bot.send_message(chat_id, match chat.lang {
        Lang::Bilingual => statics::HELP_MESSAGE_BILINGUAL,
        Lang::Chinese => statics::HELP_MESSAGE_CHINESE,
        Lang::English => statics::HELP_MESSAGE_ENGLISH,
    })
    .reply_to_message_id(message.id)
    .await?;

    respond(())
}
