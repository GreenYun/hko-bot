// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::fmt::Write;

use chrono::{Timelike, Utc};
use hko::weather::Name as WeatherName;
use num_traits::FromPrimitive;
use teloxide::{prelude::*, requests::ResponseResult, types::ParseMode};

use crate::{
    database::Connection,
    telegram::misc::start_first,
    tool::{macros::unwrap_or_execute, mix_strings, try_data, types::BilingualString},
    weather,
};

#[rustfmt::skip]
macro_rules! ch_num {
    (1) => ("一");
    (2) => ("二");
    (3) => ("三");
    (4) => ("四");
    (5) => ("五");
    (6) => ("六");
    (7) => ("七");
    (8) => ("八");
    (9) => ("九");
    (10) => ("十");
    (11) => ("十一");
    (12) => ("十二");
}

const fn chinese_hour(pm: bool, hour12: u32) -> &'static str {
    macro_rules! fmt_hour {
        {$desc:literal | $pm:literal in [$($hour:tt)+]} => {
            $(
                if pm == $pm && hour12 == $hour {
                    return concat!($desc, ch_num!($hour), "時");
                }
            )+
        };
    }

    fmt_hour! {"午夜" | false in [12]}
    fmt_hour! {"凌晨" | false in [1 2 3 4 5]}
    fmt_hour! {"上午" | false in [6 7 8 9 10 11]}
    fmt_hour! {"正午" | true in [12]}
    fmt_hour! {"下午" | true in [1 2 3 4 5]}
    fmt_hour! {"傍晚" | true in [6]}
    fmt_hour! {"晚上" | true in [7 8 9 10 11]}
    unreachable!()
}

const fn english_hour(pm: bool, hour12: u32) -> &'static str {
    macro_rules! en_hour {
        {$desc:literal | $pm:literal in [$($hour:tt)+]} => {
            $(
                if pm == $pm && hour12 == $hour {
                    return concat!(stringify!($hour), " ", $desc);
                }
            )+
        };
    }

    en_hour! {"a.m." | false in [12 1 2 3 4 5 6 7 8 9 10 11]}
    en_hour! {"p.m." | true in [12 1 2 3 4 5 6 7 8 9 10 11]}
    unreachable!()
}

fn uv_desc(i: f32) -> BilingualString {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let i = i as u32;
    match i {
        0..=2 => BilingualString {
            chinese: "低".to_owned(),
            english: "Low".to_owned(),
        },
        3..=5 => BilingualString {
            chinese: "中".to_owned(),
            english: "Moderate".to_owned(),
        },
        6..=7 => BilingualString {
            chinese: "高".to_owned(),
            english: "High".to_owned(),
        },
        8..=10 => BilingualString {
            chinese: "甚高".to_owned(),
            english: "Very high".to_owned(),
        },
        _ => BilingualString {
            chinese: "極高".to_owned(),
            english: "Extreme".to_owned(),
        },
    }
}

#[allow(clippy::too_many_lines)]
pub(super) async fn bulletin(message: Message, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
    let chat_id = message.chat.id;
    let chat = unwrap_or_execute!(db_conn.select_chat(chat_id.0).await, |e| {
        log::error!("{e}");
        return respond(());
    });
    let chat = unwrap_or_execute!(chat, || {
        return start_first(bot, chat_id).await;
    });

    let bulletin = try_data(weather::bulletin, |v| {
        (Utc::now().naive_utc() - v.update_time.naive_utc()).num_days() <= 1
    })
    .await;
    let bulletin = unwrap_or_execute!(bulletin, || {
        bot.send_message(chat_id, "Connection timed out, please try again later.")
            .reply_to_message_id(message.id)
            .await?;
        return respond(());
    });

    let (pm, hour12) = bulletin.update_time.time().hour12();
    let chi_hour = chinese_hour(pm, hour12);
    let eng_hour = english_hour(pm, hour12);

    let weather_desc = bulletin
        .weather_icon
        .into_iter()
        .map(|n| WeatherName::from_i32(n).unwrap())
        .collect::<Vec<_>>();
    let chi_weather_desc = weather_desc
        .iter()
        .map(|n| format!("{n:o}"))
        .collect::<Vec<_>>()
        .join("；");
    let eng_weather_desc = weather_desc
        .iter()
        .map(|n| format!("{n:e}"))
        .collect::<Vec<_>>()
        .join("; ");
    let uv_desc = bulletin.uv_index.map(uv_desc).unwrap_or_default();

    let mut chi_text1 = format!(
        "{chi_hour}香港天文台錄得：\n\
         氣溫：<b>{}</b> 度\n\
         相對濕度：百分之 <b>{}</b>\n\
         <b>{chi_weather_desc}</b>",
        bulletin.temperature, bulletin.humidity,
    );
    if let Some(uv_index) = bulletin.uv_index {
        chi_text1 += &format!(
            "\n\n\
             過去一小時：\n\
             京士柏錄得的平均紫外線指數：<b>{uv_index}</b>\n\
             紫外線強度屬於<b>{}</b>",
            uv_desc.chinese
        );
    }

    let mut eng_text1 = format!(
        "At {eng_hour} at Hong Kong Observatory:\n\
         Temperature: <b>{}</b> degrees Celsius\n\
         Relative humidity: <b>{}</b> per cent\n\
         <b>{eng_weather_desc}</b>",
        bulletin.temperature, bulletin.humidity,
    );
    if let Some(uv_index) = bulletin.uv_index {
        eng_text1 += &format!(
            "\n\n\
             During the past hour:\n\
             The mean UV Index recorded at King's Park: <b>{uv_index}</b>\n\
             The intensity of UV radiation is <b>{}</b>",
            uv_desc.english
        );
    }

    let text1 = BilingualString {
        chinese: chi_text1,
        english: eng_text1,
    };

    let (chi_special_tips, eng_special_tips) = bulletin
        .special_tips
        .into_iter()
        .map(|s| (s.chinese, s.english))
        .unzip::<_, _, Vec<_>, Vec<_>>();
    let chi_special_tips = chi_special_tips.join("\n\n");
    let eng_special_tips = eng_special_tips.join("\n\n");
    let special_tips = BilingualString {
        chinese: chi_special_tips,
        english: eng_special_tips,
    };

    let warning = mix_strings(bulletin.warning, &chat.lang);

    let (chi_tc, eng_tc) = bulletin
        .tropical_cyclone
        .into_iter()
        .map(|s| (s.chinese, s.english))
        .unzip::<_, _, Vec<_>, Vec<_>>();
    let chi_tc = chi_tc.join("\n\n");
    let eng_tc = eng_tc.join("\n\n");
    let tropical_cyclone = BilingualString {
        chinese: chi_tc,
        english: eng_tc,
    };

    let mut text = mix_strings(
        vec![
            text1.add_single_newline(),
            if special_tips.is_empty() {
                BilingualString::default()
            } else {
                BilingualString::new(
                    "<b>特別天氣提示：</b>".to_owned(),
                    "<b>Special Weather Tips:</b>".to_owned(),
                )
            },
            special_tips.add_single_newline(),
            if warning.is_empty() {
                BilingualString::default()
            } else {
                BilingualString::new(
                    "<b>請注意：</b>".to_owned(),
                    "<b>Please be reminded that:</b>".to_owned(),
                )
            },
        ],
        &chat.lang,
    );
    text += &warning;
    text += &mix_strings(
        vec![
            tropical_cyclone.add_single_newline(),
            bulletin.rainstorm_reminder.add_single_newline(),
        ],
        &chat.lang,
    );

    let _ = write!(text, "<i>@ {}</i>", bulletin.update_time);

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::Html)
        .reply_to_message_id(message.id)
        .await?;

    respond(())
}
