// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use hko::weather::{Current as Source, Name as WeatherName};
use tokio::sync::RwLock;

use crate::tool::types::BilingualString;

use super::{WeatherData, WeatherDataUpdater};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct BulletinUVIndex {
	pub value: f32,
	pub desc: BilingualString,
	pub period: BilingualString,
}

#[derive(Clone, Default)]
pub struct Bulletin {
	pub temperature: f32,
	pub humidity: f32,
	pub uv_index: Option<BulletinUVIndex>,
	pub weather_icon: Vec<WeatherName>,
	pub warning: Vec<BilingualString>,
	pub tropical_cyclone: Vec<BilingualString>,
	pub rainstorm_reminder: BilingualString,
	pub special_tips: Vec<BilingualString>,
	pub update_time: DateTime<FixedOffset>,
}

static STORE: OnceLock<RwLock<Bulletin>> = OnceLock::new();

impl Bulletin {
	fn new(zh: Source, en: Source) -> Self {
		let get_uv_index = || {
			let chinese = zh.uv_index.uv_index();
			let english = en.uv_index.uv_index();
			if chinese.is_none() || english.is_none() {
				return None;
			}

			let chinese = chinese.unwrap();
			let english = english.unwrap();
			let chi_period = chinese.record_desc;
			let eng_period = english.record_desc;

			let chinese = chinese.data.iter().find(|data| data.place == "京士柏");
			let english = english.data.iter().find(|data| data.place == "King's Park");
			if chinese.is_none() || english.is_none() {
				return None;
			}

			let chinese = chinese.unwrap();
			let english = english.unwrap();

			Some(BulletinUVIndex {
				value: chinese.value,
				desc: BilingualString::new(chinese.desc.clone(), english.desc.clone()),
				period: BilingualString::new(chi_period, eng_period),
			})
		};

		Self {
			temperature: en
				.temperature
				.data
				.into_iter()
				.find_map(|v| v.place.eq("Hong Kong Observatory").then_some(v.value))
				.unwrap_or_default(),
			humidity: en
				.humidity
				.data
				.into_iter()
				.find_map(|v| v.place.eq("Hong Kong Observatory").then_some(v.value))
				.unwrap_or_default(),
			uv_index: get_uv_index(),
			weather_icon: zh.icon.icon,
			warning: zh
				.warning_message
				.iter()
				.zip(en.warning_message.iter())
				.map(|(c, e)| BilingualString::new(c, e))
				.collect(),
			tropical_cyclone: zh
				.tcmessage
				.zip(en.tcmessage)
				.map(|(c, e)| c.into_iter().zip(e).map(|(c, e)| BilingualString::new(c, e)).collect())
				.unwrap_or_default(),
			rainstorm_reminder: {
				zh.rainstorm_reminder
					.and_then(|c| en.rainstorm_reminder.map(|e| BilingualString::new(c, e)))
					.unwrap_or_default()
			},
			special_tips: zh
				.special_tips
				.zip(en.special_tips)
				.map(|(c, e)| c.into_iter().zip(e).map(|(c, e)| BilingualString::new(c, e)).collect())
				.unwrap_or_default(),
			update_time: zh.update_time,
		}
	}
}

impl From<(Source, Source)> for Bulletin {
	fn from((zh, en): (Source, Source)) -> Self {
		Self::new(zh, en)
	}
}

impl WeatherData for Bulletin {
	fn get_store() -> &'static OnceLock<RwLock<Self>> {
		&STORE
	}
}

impl WeatherDataUpdater for Bulletin {
	type Source = Source;
}
