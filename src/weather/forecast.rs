// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset, NaiveDate};
use hko::weather::{nine_day::WeatherForcast, NineDay as Source};
use tokio::sync::RwLock;

use crate::tool::types::BilingualString;

use super::{WeatherData, WeatherDataUpdater};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct DailyForecast {
	pub date: NaiveDate,
	pub weather: BilingualString,
	pub wind: BilingualString,
	pub temp: (f32, f32),
	pub rh: (f32, f32),
	pub psr: BilingualString,
}

impl DailyForecast {
	fn new(zh: WeatherForcast, en: WeatherForcast) -> Self {
		let psr = zh.psr;
		let psr_zh = format!("{psr:o}");
		let psr_en = format!("{psr:e}");

		Self {
			date: zh.date,
			weather: BilingualString::new(zh.weather, en.weather),
			wind: BilingualString::new(zh.wind, en.wind),
			temp: (zh.min_temp.value, zh.max_temp.value),
			rh: (zh.min_humidity.value, zh.max_humidity.value),
			psr: BilingualString::new(psr_zh, psr_en),
		}
	}
}

#[derive(Clone, Default)]
pub struct Forecast {
	pub general_situation: BilingualString,
	pub daily: Vec<DailyForecast>,
	pub update_time: DateTime<FixedOffset>,
}

static STORE: OnceLock<RwLock<Forecast>> = OnceLock::new();

impl Forecast {
	fn new(zh: Source, en: Source) -> Self {
		Self {
			general_situation: BilingualString::new(zh.general_situation, en.general_situation),
			daily: zh
				.weather_forecast
				.into_iter()
				.zip(en.weather_forecast)
				.map(|(zh, en)| DailyForecast::new(zh, en))
				.collect(),
			update_time: zh.update_time,
		}
	}
}

impl From<(Source, Source)> for Forecast {
	fn from((zh, en): (Source, Source)) -> Self {
		Self::new(zh, en)
	}
}

impl WeatherData for Forecast {
	fn get_store() -> &'static OnceLock<RwLock<Self>> {
		&STORE
	}
}

impl WeatherDataUpdater for Forecast {
	type Source = Source;
}
