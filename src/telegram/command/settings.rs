// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, ParseMode, ReplyMarkup},
};

use crate::{
    database::{entities::chat::Chat, types::lang::Lang},
    statics,
};

pub(super) async fn settings(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    let msg1;
    let settings_lang1;
    let settings_lang;
    let msg2;

    match chat.lang {
        Lang::Bilingual => {
            msg1 = statics::SETTINGS_MESSAGE_1_BILINGUAL;
            settings_lang1 = statics::SETTINGS_MESSAGE_LANGUAGE_BILINGUAL;
            settings_lang = "<b>語言 Language</b>\n雙語 Bilingual";
            msg2 = statics::SETTINGS_MESSAGE_2_BILINGUAL;
        }
        Lang::Chinese => {
            msg1 = statics::SETTINGS_MESSAGE_1_CHINESE;
            settings_lang1 = statics::SETTINGS_MESSAGE_LANGUAGE_CHINESE;
            settings_lang = "<b>語言</b>\n中文";
            msg2 = statics::SETTINGS_MESSAGE_2_CHINESE;
        }
        Lang::English => {
            msg1 = statics::SETTINGS_MESSAGE_1_ENGLISH;
            settings_lang1 = statics::SETTINGS_MESSAGE_LANGUAGE_ENGLISH;
            settings_lang = "<b>Language</b>\nEnglish";
            msg2 = statics::SETTINGS_MESSAGE_2_ENGLISH;
        }
    }

    bot.send_message(chat_id, msg1.to_string() + "\n\n" + settings_lang + "\n\n" + msg2)
        .parse_mode(ParseMode::Html)
        .reply_markup(ReplyMarkup::inline_kb(vec![vec![InlineKeyboardButton::callback(
            settings_lang1,
            "/setlang",
        )]]))
        .await?;

    respond(())
}
