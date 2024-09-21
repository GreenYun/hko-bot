// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt::Write, sync::LazyLock};

use crate::{
	database::types::lang::Lang,
	statics::get_bilingual_str,
	tool::{data::out_dated, mix_strings, types::BilingualString},
	weather::{Briefing as Data, WeatherData as _},
};

use super::{Answer, AnswerEntry, AnswerStore};

static ANSWER: LazyLock<AnswerStore> = LazyLock::new(AnswerStore::default);

pub struct Briefing;

impl Answer for Briefing {
	async fn answer(lang: &Lang) -> Vec<String> {
		let entry = ANSWER.update_and_get(lang, update).await;
		entry.inner
	}
}

async fn update(lang: &Lang, entry: AnswerEntry) -> AnswerEntry {
	let timeout_err = get_bilingual_str!(lang, SERVER_ERROR_TIMEOUT);

	let data = Data::get().await.filter(|data| !out_dated(data.update_time.to_utc()));

	let Some(data) = data else {
		return AnswerEntry::new_err(timeout_err);
	};

	if entry.update_time >= data.update_time {
		return entry;
	}

	let update_time = data.update_time;

	let mut inner = mix_strings(lang, &[
		data.general_situation.add_single_newline(),
		data.tc_info.add_single_newline(),
		data.fire_danger_warning.add_single_newline(),
		("<b>".to_string() + data.forecast_period + "</b>"),
		data.forecast_desc.add_single_newline(),
		(BilingualString::new("展望：", "Outlook: ") + data.outlook).add_single_newline(),
	]);

	write!(inner, "\n\n<i>@ {}</i>", data.update_time).ok();
	let inner = vec![inner];
	AnswerEntry::new(inner, update_time)
}
