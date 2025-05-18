// Copyright (c) 2024 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt::Write, sync::LazyLock};

use chrono::Datelike;

use crate::{
	answer::macros::zh_weekday,
	database::types::lang::Lang,
	statics::get_bilingual_str,
	tool::{data::out_dated, mix_strings, types::BilingualString},
	weather::{Forecast as Data, WeatherData as _, forecast::DailyForecast},
};

use super::{Answer, AnswerEntry, AnswerStore, macros::zh_num};

static ANSWER: LazyLock<AnswerStore> = LazyLock::new(AnswerStore::default);

pub struct Forecast;

impl Answer for Forecast {
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

	let gs = mix_strings(lang, &[
		BilingualString::new("<b>天氣概況：</b>", "<b>General Situation:</b>"),
		data.general_situation.add_single_newline(),
	]);

	let ut = format!("<i>@ {}</i>", data.update_time);

	let mut inner = data.daily.iter().map(|d| daily_string(lang, d)).collect::<Vec<_>>();

	inner.insert(0, gs);
	inner.push(ut);
	AnswerEntry::new(inner, update_time)
}

fn daily_string(lang: &Lang, data: &DailyForecast) -> String {
	let mut zh = String::new();
	let mut en = String::new();

	let mon = data.date.month();
	let day = data.date.day();
	let weekday = data.date.weekday();

	if !matches!(lang, Lang::English) {
		let mon = zh_num!(mon);
		let day = zh_num!(day);
		let weekday = zh_weekday!(weekday);

		zh.reserve(128);
		writeln!(zh, "<b>{mon}月{day}日（{weekday}）</b>").ok();
		writeln!(zh, "風　：{:x}", data.wind).ok();
		writeln!(zh, "天氣：{:x}", data.weather).ok();
		writeln!(zh, "氣溫：{} 至 {} 度。", data.temp.0, data.temp.1).ok();
		writeln!(zh, "相對濕度：百分之 {} 至 {}。", data.rh.0, data.rh.1).ok();
		write!(zh, "顯著降雨概率：{:x}", data.psr).ok();
	}

	if !matches!(lang, Lang::Chinese) {
		en.reserve(128);
		writeln!(en, "<b>{}</b>", data.date.format("%d %B (%A)")).ok();
		writeln!(en, "Wind: {:e}", data.wind).ok();
		writeln!(en, "Weather: {:e}", data.weather).ok();
		writeln!(en, "Temp Range: {} - {} C", data.temp.0, data.temp.1).ok();
		writeln!(en, "R.H. Range: {} - {} Per Cent", data.rh.0, data.rh.1).ok();
		write!(en, "PSR: {:e}", data.psr).ok();
	}

	lang.map(format!("{zh}\n\n{en}"), zh, en)
}
