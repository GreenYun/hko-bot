// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::{fmt::Write, time::Duration};

use chrono::Utc;
use teloxide::{prelude::*, requests::ResponseResult, types::ParseMode};
use tokio::time;

use crate::{
    database::Connection,
    telegram::misc::start_first,
    tool::{macros::unwrap_or_excute, mix_strings, types::BilingualString},
};

pub(super) async fn briefing(message: Message, bot: AutoSend<Bot>, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;
    let chat = unwrap_or_excute!(db_conn.select_chat(chat_id.0).await, |e| {
        log::error!("{:?}", e);
        return respond(());
    });

    if chat.is_none() {
        return start_first(bot, chat_id).await;
    };

    let chat = chat.unwrap();
    let lang = chat.lang;

    let mut briefing = Default::default();
    for i in 0..3 {
        let arc = crate::weather::briefing();
        briefing = {
            let lock = arc.read().await;
            lock.clone()
        };

        if (Utc::now().naive_utc() - briefing.update_time.naive_utc()).num_days() <= 1 {
            break;
        }

        if i >= 2 {
            bot.send_message(chat_id, "Connection timed out, please try again later.")
                .reply_to_message_id(message.id)
                .await?;
            return respond(());
        }

        time::sleep(Duration::from_secs(10)).await;
    }

    let mut text = mix_strings(
        vec![
            briefing.general_situation,
            "<b>".to_owned()
                + briefing.forecast_period
                + "</b>\n"
                + briefing.forecast_desc
                + BilingualString::new("\n展望", "\n")
                + briefing.outlook,
            briefing.tc_info,
            briefing.fire_danger_warning,
        ],
        lang,
    );

    let _ = write!(text, "<i>@ {}</i>", briefing.update_time);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(message.id)
        .await?;

    respond(())
}
