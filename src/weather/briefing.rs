// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use hko::weather::Local;
use tokio::sync::RwLock;

use crate::tool::types::BilingualString;

use super::{WeatherData, WeatherDataUpdater};

#[derive(Clone, Default)]
pub struct Briefing {
	pub general_situation: BilingualString,
	pub forecast_period: BilingualString,
	pub forecast_desc: BilingualString,
	pub outlook: BilingualString,
	pub tc_info: BilingualString,
	pub fire_danger_warning: BilingualString,
	pub update_time: DateTime<FixedOffset>,
}

static BRIEFING_STORE: OnceLock<RwLock<Briefing>> = OnceLock::new();

impl Briefing {
	fn new(chinese: Local, english: Local) -> Self {
		Self {
			general_situation: BilingualString::new(chinese.general_situation, english.general_situation),
			forecast_period: BilingualString::new(chinese.forecast_period, english.forecast_period),
			forecast_desc: BilingualString::new(chinese.forecast_desc, english.forecast_desc),
			outlook: BilingualString::new(chinese.outlook, english.outlook),
			tc_info: BilingualString::new(chinese.tc_info, english.tc_info),
			fire_danger_warning: BilingualString::new(chinese.fire_danger_warning, english.fire_danger_warning),
			update_time: chinese.update_time,
		}
	}
}

impl WeatherData for Briefing {
	async fn get() -> Option<Self> {
		if let Some(lock) = BRIEFING_STORE.get() {
			let lock = lock.read().await;
			Some(lock.clone())
		} else {
			None
		}
	}
}

impl WeatherDataUpdater for Briefing {
	type Source = Local;

	async fn update(chinese: Local, english: Local) {
		let translated = Self::new(chinese, english);
		if let Some(lock) = BRIEFING_STORE.get() {
			let mut lock = lock.write().await;
			*lock = translated;
		} else {
			BRIEFING_STORE.set(RwLock::new(translated)).ok();
		}
	}
}
