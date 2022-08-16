// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{prelude::*, requests::ResponseResult, types::ParseMode};

use crate::{
    database::{entities::chat::Chat, types::lang::Lang, Connection},
    macros::unwrap_or_excute,
    statics,
};

pub(super) async fn start(message: Message, bot: AutoSend<Bot>, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    if let Some(chat) = unwrap_or_excute!(
        db_conn.select_chat(chat_id.0).await,
        Err | e | {
            log::error!("{:?}", e);
            return respond(());
        }
    ) {
        let zh_text = "喂，老友。";
        let en_text = "Hi, my old friend.";
        bot.send_message(chat_id, match chat.lang {
            Lang::Chinese => zh_text.to_owned(),
            Lang::English => en_text.to_owned(),
            Lang::Bilingual => format!("{}\n{}", zh_text, en_text),
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

    unwrap_or_excute!(
        db_conn.insert_chat(&chat).await,
        Err | e | {
            log::error!("{:?}", e);
            return respond(());
        }
    );

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
