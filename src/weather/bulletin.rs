// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use hko::weather::{Current, Name as WeatherName};
use tokio::sync::RwLock;

use crate::tool::types::BilingualString;

use super::{WeatherData, WeatherDataUpdater};

#[derive(Clone, Default)]
pub struct Bulletin {
    pub temperature: f32,
    pub humidity: f32,
    pub uv_index: Option<f32>,
    pub weather_icon: Vec<WeatherName>,
    pub warning: Vec<BilingualString>,
    pub tropical_cyclone: Vec<BilingualString>,
    pub rainstorm_reminder: BilingualString,
    pub special_tips: Vec<BilingualString>,
    pub update_time: DateTime<FixedOffset>,
}

static BULLETIN_STORE: OnceLock<RwLock<Bulletin>> = OnceLock::new();

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
        let translated = Self::translate(chinese, english);
        if let Some(lock) = BULLETIN_STORE.get() {
            let mut lock = lock.write().await;
            *lock = translated;
        } else {
            BULLETIN_STORE.set(RwLock::new(translated)).ok();
        }
    }

    fn translate(chinese: Self::Source, english: Self::Source) -> Self {
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
            uv_index: english.uv_index.uv_index().and_then(|v| {
                v.data
                    .into_iter()
                    .find_map(|v| v.place.eq("King's Park").then_some(v.value))
            }),
            weather_icon: chinese.icon.icon,
            warning: chinese
                .warning_message
                .zip(english.warning_message)
                .map(|(c, e)| c.into_iter().zip(e).map(|(c, e)| BilingualString::new(c, e)).collect())
                .unwrap_or_default(),
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
