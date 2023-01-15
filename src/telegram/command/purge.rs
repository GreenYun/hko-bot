// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult};

use crate::{database::Connection, telegram::misc::start_first};

pub(super) async fn purge(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
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

    if let Err(e) = db_conn.delete_chat(chat.id).await {
        log::error!("{e}");
        return respond(());
    };

    bot.send_message(chat_id, "\u{1F44B}").await?;

    respond(())
}
