// Copyright (c) 2024 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{fmt::Write, sync::LazyLock};

use chrono::Timelike as _;

use crate::{
	database::types::lang::Lang,
	statics::get_bilingual_str,
	tool::{
		data::out_dated,
		mix_string, mix_strings,
		types::{BilingualStr, BilingualString},
	},
	weather::{Bulletin as Data, WeatherData as _},
};

use super::{Answer, AnswerEntry, AnswerStore, macros::zh_num};

static ANSWER: LazyLock<AnswerStore> = LazyLock::new(AnswerStore::default);

pub struct Bulletin;

impl Answer for Bulletin {
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

	let inner = to_string(&data, lang);
	AnswerEntry::new(inner, update_time)
}

fn to_string(data: &Data, lang: &Lang) -> Vec<String> {
	static SPECIAL_WEATHER_TIPS: BilingualStr =
		BilingualStr::new("<b>特別天氣提示：</b>", "<b>Special Weather Tips:</b>");
	static WEATHER_WARNING: BilingualStr = BilingualStr::new("<b>請注意：</b>", "<b>Please be reminded that:</b>");

	let (pm, hour12) = data.update_time.time().hour12();
	let chi_hour = chinese_hour(pm, hour12);
	let eng_hour = english_hour(pm, hour12);

	let (chi_temp, chi_uv) = if matches!(lang, Lang::English) {
		(String::new(), String::new())
	} else {
		let chi_weather_desc = data.weather_icon.iter().map(|n| format!("{n:o}")).collect::<Vec<_>>().join("\u{ff1b}");
		let chi_temp = format!(
			"\
			{chi_hour}香港天文台錄得：\n\
        	氣溫：<b>{}</b> 度\n\
        	相對濕度：百分之 <b>{}</b>\n\
        	<b>{chi_weather_desc}</b>",
			data.temperature, data.humidity,
		);
		let chi_uv = data.uv_index.clone().map_or_else(String::new, |uv_index| {
			format!(
				"\
				{:x}：\n\
            	京士柏錄得的平均紫外線指數：<b>{}</b>\n\
            	紫外線強度：<b>{:x}</b>",
				uv_index.period, uv_index.value, uv_index.desc
			)
		});
		(chi_temp, chi_uv)
	};

	let (eng_temp, eng_uv) = if matches!(lang, Lang::Chinese) {
		(String::new(), String::new())
	} else {
		let eng_weather_desc = data.weather_icon.iter().map(|n| format!("{n:e}")).collect::<Vec<_>>().join("; ");
		let eng_temp = format!(
			"\
			At {eng_hour} at Hong Kong Observatory:\n\
    		Air temperature: <b>{}</b> degrees Celsius\n\
        	Relative humidity: <b>{}</b> per cent\n\
        	<b>{eng_weather_desc}</b>",
			data.temperature, data.humidity,
		);
		let eng_uv = data.uv_index.clone().map_or_else(String::new, |uv_index| {
			format!(
				"\
				{:e}:\n\
             	The mean UV Index recorded at King's Park: <b>{}</b>\n\
             	Intensity of UV radiation: <b>{:e}</b>",
				uv_index.period, uv_index.value, uv_index.desc
			)
		});
		(eng_temp, eng_uv)
	};

	let mut text = mix_strings(lang, &[
		BilingualString::new(chi_temp, eng_temp).add_single_newline(),
		BilingualString::new(chi_uv, eng_uv).add_single_newline(),
		data.rainstorm_reminder.clone(),
	]);
	text.reserve(4096 - text.len());

	if !data.special_tips.is_empty() {
		let special_tips: Vec<_> =
			data.special_tips.iter().map(Clone::clone).map(BilingualString::add_single_newline).collect();
		write!(text, "\n\n{}\n\n{}", &mix_string(lang, &SPECIAL_WEATHER_TIPS), &mix_strings(lang, &special_tips)).ok();
	}

	if !data.warning.is_empty() {
		let warning: Vec<_> = data.warning.iter().map(Clone::clone).map(BilingualString::add_single_newline).collect();
		write!(text, "\n\n{}\n\n{}", mix_string(lang, &WEATHER_WARNING), mix_strings(lang, &warning)).ok();
	}

	if !data.tropical_cyclone.is_empty() {
		let tropical_cyclone: Vec<_> =
			data.tropical_cyclone.iter().map(Clone::clone).map(BilingualString::add_single_newline).collect();
		write!(text, "\n\n{}", &mix_strings(lang, &tropical_cyclone)).ok();
	}

	if matches!(lang, Lang::Bilingual) && text.len() > 4000 {
		let mut ch = to_string(data, &Lang::Chinese);
		let en = to_string(data, &Lang::English);
		ch.extend_from_slice(&en);
		ch
	} else {
		write!(text, "\n\n<i>@ {}</i>", data.update_time).ok();
		text.shrink_to_fit();
		vec![text]
	}
}

const fn chinese_hour(pm: bool, hour12: u32) -> &'static str {
	macro_rules! fmt_zh_hour {
        {$desc:literal | $pm:literal in [$($hour:tt)+]} => {
            $(
                if pm == $pm && hour12 == $hour {
                    return concat!($desc, zh_num!($hour), "時");
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
