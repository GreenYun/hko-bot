// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt::Write, sync::LazyLock};

use chrono::DateTime;

use crate::{
	database::types::lang::Lang,
	statics::get_bilingual_str,
	tool::mix_strings,
	weather::{Warning as Data, WeatherData as _},
};

use super::{Answer, AnswerEntry, AnswerStore};

static ANSWER: LazyLock<AnswerStore> = LazyLock::new(AnswerStore::default);

pub struct Warning;

impl Answer for Warning {
	async fn answer(lang: &Lang) -> Vec<String> {
		let entry = ANSWER.update_and_get(lang, update).await;
		entry.inner
	}
}

async fn update(lang: &Lang, entry: AnswerEntry) -> AnswerEntry {
	let timeout_err = get_bilingual_str!(lang, SERVER_ERROR_TIMEDOUT);

	let data = Data::get().await;

	let Some(data) = data else {
		return AnswerEntry::new_err(timeout_err);
	};

	if data.pieces.is_empty() {
		return AnswerEntry::new(vec![], DateTime::default());
	}

	let update_time = data.pieces.iter().max_by_key(|w| w.update_time).map(|w| w.update_time).unwrap_or_default();

	if entry.update_time >= update_time {
		return entry;
	}

	let inner = to_strings(data, lang);
	AnswerEntry::new(inner, update_time)
}

fn to_strings(data: Data, lang: &Lang) -> Vec<String> {
	if data.pieces.is_empty() {
		return Vec::new();
	}

	let mut pieces = Vec::new();

	for p in data.pieces {
		let mut list = vec!["<b>".to_string() + p.name + "</b>"];
		list.extend_from_slice(&p.contents);

		let mut text = mix_strings(lang, &list);

		if matches!(lang, Lang::Bilingual) && text.len() > 4000 {
			text = mix_strings(&Lang::Chinese, &list);
			write!(text, "\n\n<i>@ {}</i>", p.update_time).ok();
			pieces.push(text);

			text = mix_strings(&Lang::English, &list);
		}

		write!(text, "\n\n<i>@ {}</i>", p.update_time).ok();
		pieces.push(text);
	}

	pieces
}
