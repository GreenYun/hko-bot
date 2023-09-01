// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use chrono::Utc;
use teloxide::{prelude::*, requests::ResponseResult, types::ParseMode};

use crate::{
    database::Connection,
    telegram::misc::start_first,
    tool::{mix_strings, try_data, types::BilingualString},
    weather,
};

pub(super) async fn briefing(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
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

    let Some(briefing) = try_data(weather::briefing, |v| {
        (Utc::now().naive_utc() - v.update_time.naive_utc()).num_days() <= 1
    })
    .await
    else {
        bot.send_message(chat_id, "Connection timed out, please try again later.")
            .reply_to_message_id(message.id)
            .await?;
        return respond(());
    };

    let mut text = mix_strings(
        &[
            briefing.general_situation.add_single_newline(),
            ("<b>".to_owned()
                + briefing.forecast_period
                + "</b>\n"
                + briefing.forecast_desc
                + BilingualString::new("\n展望", "\nOutlook: ")
                + briefing.outlook)
                .add_single_newline(),
            briefing.tc_info.add_single_newline(),
            briefing.fire_danger_warning.add_single_newline(),
        ],
        &chat.lang,
    );

    write!(text, "<i>@ {}</i>", briefing.update_time).ok();

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(message.id)
        .await?;

    respond(())
}
