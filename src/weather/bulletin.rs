// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use chrono::{DateTime, FixedOffset};
use hko::weather::Current;

use crate::tool::{macros::unwrap_or_execute, types::BilingualString};

use super::macros::impl_update;

#[derive(Clone, Default)]
pub struct Bulletin {
    pub temperature: f32,
    pub humidity: f32,
    pub uv_index: f32,
    pub weather_icon: Vec<i32>,
    pub warning: Vec<BilingualString>,
    pub tropical_cyclone: Vec<BilingualString>,
    pub rainstorm_reminder: BilingualString,
    pub special_tips: Vec<BilingualString>,
    pub update_time: DateTime<FixedOffset>,
}

impl Bulletin {
    pub fn new(chinese: Current, english: Current) -> Self {
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
            uv_index: english
                .uv_index
                .uv_index()
                .and_then(|v| {
                    v.data
                        .into_iter()
                        .find_map(|v| v.place.eq("King's Park").then_some(v.value))
                })
                .unwrap_or_default(),
            weather_icon: chinese.icon.icon,
            warning: chinese
                .warning_message
                .zip(english.warning_message)
                .map(|(c, e)| {
                    c.into_iter()
                        .zip(e.into_iter())
                        .map(|(c, e)| BilingualString::new(c, e))
                        .collect()
                })
                .unwrap_or_default(),
            tropical_cyclone: chinese
                .tcmessage
                .zip(english.tcmessage)
                .map(|(c, e)| {
                    c.into_iter()
                        .zip(e.into_iter())
                        .map(|(c, e)| BilingualString::new(c, e))
                        .collect()
                })
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
                .map(|(c, e)| {
                    c.into_iter()
                        .zip(e.into_iter())
                        .map(|(c, e)| BilingualString::new(c, e))
                        .collect()
                })
                .unwrap_or_default(),
            update_time: chinese.update_time,
        }
    }
}

// pub async fn update() {
//     let chinese = unwrap_or_execute!(fetch(Lang::TC).await, |e| {
//         log::error!("{}", e);
//         return;
//     });

//     let english = unwrap_or_execute!(fetch(Lang::EN).await, |e| {
//         log::error!("{}", e);
//         return;
//     });

//     let b = Bulletin::new(chinese, english);

//     {
//         let arc = bulletin();
//         let mut lock = arc.write().await;
//         *lock = b;
//     }
// }

impl_update!(bulletin);
