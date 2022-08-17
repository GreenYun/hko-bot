// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult};

use crate::{
    database::{types::lang::Lang, Connection},
    statics,
    telegram::misc::start_first,
    tool::macros::unwrap_or_excute,
};

pub(super) async fn help(message: Message, bot: AutoSend<Bot>, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    match unwrap_or_excute!(db_conn.select_chat(chat_id.0).await, |e| {
        log::error!("{:?}", e);
        return respond(());
    }) {
        Some(chat) => {
            bot.send_message(chat_id, match chat.lang {
                Lang::Bilingual => statics::HELP_MESSAGE_BILINGUAL,
                Lang::Chinese => statics::HELP_MESSAGE_CHINESE,
                Lang::English => statics::HELP_MESSAGE_ENGLISH,
            })
            .reply_to_message_id(message.id)
            .await?;

            respond(())
        }

        None => start_first(bot, chat_id).await,
    }
}
