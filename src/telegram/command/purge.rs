// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult};

use crate::{database::Connection, telegram::misc::start_first, tool::macros::unwrap_or_execute};

pub(super) async fn purge(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    let chat = unwrap_or_execute!(db_conn.select_chat(chat_id.0).await, |e| {
        log::error!("{e}");
        return respond(());
    });
    let chat = unwrap_or_execute!(chat, || {
        return start_first(bot, chat_id).await;
    });

    unwrap_or_execute!(db_conn.delete_chat(chat.id).await, |e| {
        log::error!("{e}");
        return respond(());
    });

    bot.send_message(chat_id, "\u{1F44B}").await?;

    respond(())
}
