// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use chrono::Utc;

use crate::{
    database::types::lang::Lang,
    tool::{mix_strings, try_data, types::BilingualString},
    weather,
};

pub async fn to_string(lang: &Lang) -> String {
    let Some(briefing) = try_data(weather::briefing, |v| {
        (Utc::now().naive_utc() - v.update_time.naive_utc()).num_days() <= 1
    })
    .await
    else {
        return "Connection timed out, please try again later.".into();
    };

    let mut text = mix_strings(
        &[
            briefing.general_situation.add_single_newline(),
            ("<b>".to_string()
                + briefing.forecast_period
                + "</b>\n"
                + briefing.forecast_desc
                + BilingualString::new("\n展望", "\nOutlook: ")
                + briefing.outlook)
                .add_single_newline(),
            briefing.tc_info.add_single_newline(),
            briefing.fire_danger_warning.add_single_newline(),
        ],
        lang,
    );

    write!(text, "<i>@ {}</i>", briefing.update_time).ok();

    text
}
