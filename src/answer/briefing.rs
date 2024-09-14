// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt::Write, sync::OnceLock};

use chrono::{DateTime, FixedOffset};
use tokio::sync::RwLock;

use crate::{
	database::types::lang::Lang,
	statics::get_bilingual_str,
	tool::{data::out_dated, mix_strings, types::BilingualString},
	weather::{Briefing as Data, WeatherData as _},
};

use super::{Answer, AnswerStore};

static ANSWER_BI: OnceLock<RwLock<AnswerStore>> = OnceLock::new();
static ANSWER_EN: OnceLock<RwLock<AnswerStore>> = OnceLock::new();
static ANSWER_ZH: OnceLock<RwLock<AnswerStore>> = OnceLock::new();

pub struct Briefing;

impl Answer for Briefing {
	async fn answer(lang: &Lang) -> Vec<String> {
		update_and_get(lang).await
	}
}

pub async fn update_and_get(lang: &Lang) -> Vec<String> {
	let ol = lang.map(&ANSWER_BI, &ANSWER_ZH, &ANSWER_EN);
	let answer = ol.get_or_init(|| RwLock::new(AnswerStore::default()));

	let old = {
		let old = answer.read().await;
		old.clone()
	};

	if let Some(new) = update(lang, &old.update_time).await {
		let mut answer = answer.write().await;
		*answer = new;
		return answer.inner.clone();
	}

	old.inner
}

pub async fn update(lang: &Lang, last_update: &DateTime<FixedOffset>) -> Option<AnswerStore> {
	let data = Data::get().await;

	let Some(data) = data else {
		return AnswerStore {
			inner: vec![get_bilingual_str!(lang, SERVER_ERROR_TIMEDOUT).into()],
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
		data.tc_info.add_single_newline(),
		data.fire_danger_warning.add_single_newline(),
		("<b>".to_string() + data.forecast_period + "</b>"),
		data.forecast_desc.add_single_newline(),
		(BilingualString::new("展望：", "Outlook: ") + data.outlook).add_single_newline(),
	]);

	write!(inner, "\n\n<i>@ {}</i>", data.update_time).ok();

	AnswerStore::new(vec![inner], data.update_time).into()
}
