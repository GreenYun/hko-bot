// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use chrono::Timelike;

use crate::{
    database::types::lang::Lang,
    statics::get_bilingual_str,
    tool::{data::out_dated, mix_strings, types::BilingualString},
    weather::{Bulletin as Data, WeatherData},
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
    macro_rules! fmt_zh_hour {
        {$desc:literal | $pm:literal in [$($hour:tt)+]} => {
            $(
                if pm == $pm && hour12 == $hour {
                    return concat!($desc, ch_num!($hour), "時");
                }
            )+
        };
    }

    fmt_zh_hour! {"午夜" | false in [12]}
    fmt_zh_hour! {"凌晨" | false in [1 2 3 4 5]}
    fmt_zh_hour! {"上午" | false in [6 7 8 9 10 11]}
    fmt_zh_hour! {"正午" | true in [12]}
    fmt_zh_hour! {"下午" | true in [1 2 3 4 5]}
    fmt_zh_hour! {"傍晚" | true in [6]}
    fmt_zh_hour! {"晚上" | true in [7 8 9 10 11]}
    unreachable!()
}

const fn english_hour(pm: bool, hour12: u32) -> &'static str {
    macro_rules! fmt_en_hour {
        {$desc:literal | $pm:literal in [$($hour:tt)+]} => {
            $(
                if pm == $pm && hour12 == $hour {
                    return concat!(stringify!($hour), " ", $desc);
                }
            )+
        };
    }

    fmt_en_hour! {"a.m." | false in [12 1 2 3 4 5 6 7 8 9 10 11]}
    fmt_en_hour! {"p.m." | true in [12 1 2 3 4 5 6 7 8 9 10 11]}
    unreachable!()
}

fn uv_desc(i: f32) -> BilingualString {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let i = i as u32;
    match i {
        0..=2 => BilingualString::new("低", "Low"),
        3..=5 => BilingualString::new("中", "Moderate"),
        6..=7 => BilingualString::new("高", "High"),
        8..=10 => BilingualString::new("甚高", "Very high"),
        _ => BilingualString::new("極高", "Extreme"),
    }
}

#[allow(clippy::too_many_lines)]
pub async fn to_string(lang: &Lang) -> String {
    let Some(bulletin) = Data::get().await else {
        return get_bilingual_str!(lang, SERVER_ERROR_TIMEDOUT).into();
    };

    if out_dated(bulletin.update_time.to_utc()) {
        return get_bilingual_str!(lang, SERVER_ERROR_TIMEDOUT).into();
    }

    let (pm, hour12) = bulletin.update_time.time().hour12();
    let chi_hour = chinese_hour(pm, hour12);
    let eng_hour = english_hour(pm, hour12);

    let chi_weather_desc = bulletin
        .weather_icon
        .iter()
        .map(|n| format!("{n:o}"))
        .collect::<Vec<_>>()
        .join("\u{ff1b}");
    let eng_weather_desc = bulletin
        .weather_icon
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
             紫外線強度屬於<b>{uv_desc:x}</b>",
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
             The intensity of UV radiation is <b>{uv_desc:e}</b>",
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

    let warning = mix_strings(lang, &bulletin.warning);

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

    let mut text = mix_strings(lang, &[
        text1.add_single_newline(),
        (!special_tips.is_empty())
            .then_some(BilingualString::new(
                "<b>特別天氣提示：</b>",
                "<b>Special Weather Tips:</b>",
            ))
            .unwrap_or_default(),
        special_tips.add_single_newline(),
        (!warning.is_empty())
            .then_some(BilingualString::new(
                "<b>請注意：</b>",
                "<b>Please be reminded that:</b>",
            ))
            .unwrap_or_default(),
    ]);
    text += &warning;
    text += &mix_strings(lang, &[
        tropical_cyclone.add_single_newline(),
        bulletin.rainstorm_reminder.add_single_newline(),
    ]);

    write!(text, "<i>@ {}</i>", bulletin.update_time).ok();

    text
}
