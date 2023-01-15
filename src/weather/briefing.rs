// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use chrono::{DateTime, FixedOffset};
use hko::weather::Local;

use crate::tool::types::BilingualString;

use super::macros::impl_update;

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

impl Briefing {
    pub fn new(chinese: Local, english: Local) -> Self {
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

impl_update!(briefing);
