// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt::Write, sync::OnceLock};

use chrono::{DateTime, FixedOffset, Utc};
use tokio::sync::RwLock;

use crate::{
    database::types::lang::Lang,
    statics::get_bilingual_str,
    tool::{
        data::{out_dated, out_minuted},
        mix_strings,
        types::BilingualString,
    },
    weather::{Briefing as Data, WeatherData},
};

use super::{Answer, AnswerStore};

pub struct Briefing;

impl Answer for Briefing {
    async fn answer(lang: &Lang) -> String {
        update_and_get(lang).await
    }
}

static ANSWER_BI: OnceLock<RwLock<AnswerStore>> = OnceLock::new();
static ANSWER_EN: OnceLock<RwLock<AnswerStore>> = OnceLock::new();
static ANSWER_ZH: OnceLock<RwLock<AnswerStore>> = OnceLock::new();

static LAST_CHECK: OnceLock<RwLock<DateTime<Utc>>> = OnceLock::new();

pub async fn update_and_get(lang: &Lang) -> String {
    let ol = lang.map(&ANSWER_BI, &ANSWER_ZH, &ANSWER_EN);
    let answer = ol.get_or_init(|| RwLock::new(AnswerStore::default()));

    let old = {
        let old = answer.read().await;
        old.clone()
    };

    if let Some(new) = update(&Lang::Chinese, &old.update_time).await {
        let mut answer = answer.write().await;
        *answer = new;
        return answer.inner.clone();
    }

    old.inner
}

pub async fn update(lang: &Lang, last_update: &DateTime<FixedOffset>) -> Option<AnswerStore> {
    {
        let last_check = LAST_CHECK.get_or_init(|| RwLock::new(DateTime::default()));
        let mut last_check = last_check.write().await;
        let now = out_minuted(*last_check)?;
        *last_check = now;
    }

    let data = Data::get().await;

    let Some(data) = data else {
        return AnswerStore {
            inner: get_bilingual_str!(lang, SERVER_ERROR_TIMEDOUT).into(),
            update_time: DateTime::UNIX_EPOCH.into(),
        }
        .into();
    };

    if out_dated(data.update_time.to_utc()) {
        return None;
    }

    if last_update >= &data.update_time {
        return None;
    }

    let mut inner = mix_strings(lang, &[
        data.general_situation.add_single_newline(),
        ("<b>".to_string()
            + data.forecast_period
            + "</b>\n"
            + data.forecast_desc
            + BilingualString::new("\n展望", "\nOutlook: ")
            + data.outlook)
            .add_single_newline(),
        data.tc_info.add_single_newline(),
        data.fire_danger_warning.add_single_newline(),
    ]);

    write!(inner, "<i>@ {}</i>", data.update_time).ok();

    AnswerStore {
        inner,
        update_time: data.update_time,
    }
    .into()
}
