// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::prelude::*;

use crate::database::{entities::chat::Chat, Connection};

pub(super) async fn purge(message: Message, bot: Bot, chat: Chat, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    if let Err(e) = db_conn.delete_chat(chat.id).await {
        log::error!("{e}");
        return respond(());
    };

    bot.send_message(chat_id, "\u{1F44B}").await?;

    respond(())
}
