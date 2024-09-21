// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use hko::weather::Local as Source;
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

static STORE: OnceLock<RwLock<Briefing>> = OnceLock::new();

impl Briefing {
	fn new(zh: Source, en: Source) -> Self {
		Self {
			general_situation: BilingualString::new(zh.general_situation, en.general_situation),
			forecast_period: BilingualString::new(zh.forecast_period, en.forecast_period),
			forecast_desc: BilingualString::new(zh.forecast_desc, en.forecast_desc),
			outlook: BilingualString::new(zh.outlook, en.outlook),
			tc_info: BilingualString::new(zh.tc_info, en.tc_info),
			fire_danger_warning: BilingualString::new(zh.fire_danger_warning, en.fire_danger_warning),
			update_time: zh.update_time,
		}
	}
}

impl From<(Source, Source)> for Briefing {
	fn from((zh, en): (Source, Source)) -> Self {
		Self::new(zh, en)
	}
}

impl WeatherData for Briefing {
	fn get_store() -> &'static OnceLock<RwLock<Self>> {
		&STORE
	}
}

impl WeatherDataUpdater for Briefing {
	type Source = Source;
}
