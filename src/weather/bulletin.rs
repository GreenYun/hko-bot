// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use hko::weather::{Current, Name as WeatherName};
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

static BULLETIN_STORE: OnceLock<RwLock<Bulletin>> = OnceLock::new();

impl Bulletin {
	fn new(chinese: Current, english: Current) -> Self {
		let get_uv_index = || {
			let chinese = chinese.uv_index.uv_index();
			let english = english.uv_index.uv_index();
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
			temperature: english
				.temperature
				.data
				.into_iter()
				.find_map(|v| v.place.eq("Hong Kong Observatory").then_some(v.value))
				.unwrap_or_default(),
			humidity: english
				.humidity
				.data
				.into_iter()
				.find_map(|v| v.place.eq("Hong Kong Observatory").then_some(v.value))
				.unwrap_or_default(),
			uv_index: get_uv_index(),
			weather_icon: chinese.icon.icon,
			warning: chinese
				.warning_message
				.iter()
				.zip(english.warning_message.iter())
				.map(|(c, e)| BilingualString::new(c, e))
				.collect(),
			tropical_cyclone: chinese
				.tcmessage
				.zip(english.tcmessage)
				.map(|(c, e)| c.into_iter().zip(e).map(|(c, e)| BilingualString::new(c, e)).collect())
				.unwrap_or_default(),
			rainstorm_reminder: {
				chinese
					.rainstorm_reminder
					.and_then(|c| english.rainstorm_reminder.map(|e| BilingualString::new(c, e)))
					.unwrap_or_default()
			},
			special_tips: chinese
				.special_tips
				.zip(english.special_tips)
				.map(|(c, e)| c.into_iter().zip(e).map(|(c, e)| BilingualString::new(c, e)).collect())
				.unwrap_or_default(),
			update_time: chinese.update_time,
		}
	}
}

impl WeatherData for Bulletin {
	async fn get() -> Option<Self> {
		if let Some(lock) = BULLETIN_STORE.get() {
			let lock = lock.read().await;
			Some(lock.clone())
		} else {
			None
		}
	}
}

impl WeatherDataUpdater for Bulletin {
	type Source = Current;

	async fn update(chinese: Self::Source, english: Self::Source) {
		let translated = Self::new(chinese, english);
		if let Some(lock) = BULLETIN_STORE.get() {
			let mut lock = lock.write().await;
			*lock = translated;
		} else {
			BULLETIN_STORE.set(RwLock::new(translated)).ok();
		}
	}
}
