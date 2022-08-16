// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult};

use crate::{database::Connection, macros::unwrap_or_excute, telegram::misc::start_first};

pub(super) async fn purge(message: Message, bot: AutoSend<Bot>, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    match unwrap_or_excute!(db_conn.select_chat(chat_id.0).await, |e| {
        log::error!("{:?}", e);
        return respond(());
    }) {
        Some(chat) => {
            unwrap_or_excute!(db_conn.delete_chat(chat.id).await, |e| {
                log::error!("{:?}", e);
                return respond(());
            });

            bot.send_message(chat_id, "\u{1F44B}").await?;
            respond(())
        }

        None => start_first(bot, chat_id).await,
    }
}
