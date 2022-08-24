// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use chrono::{DateTime, FixedOffset};
use hko::weather::{warning::info::InfoDetail, Info};

use crate::tool::{macros::unwrap_or_execute, types::BilingualString};

use super::macros::impl_update;

#[derive(Clone, Default)]
pub struct WarningPiece {
    pub name: BilingualString,
    pub contents: Vec<BilingualString>,
    pub update_time: DateTime<FixedOffset>,
}

impl WarningPiece {
    pub fn new(chinese: InfoDetail, english: InfoDetail) -> Self {
        let mut chinese_name = format!("{:o}", chinese.code);
        let mut english_name = format!("{:e}", english.code);

        if let Some(code) = chinese.subtype {
            chinese_name.push_str(&format!("：{:o}", code));
            english_name.push_str(&format!(": {:e}", code));
        }

        Self {
            name: BilingualString::new(chinese_name, english_name),
            contents: chinese
                .contents
                .zip(english.contents)
                .map(|(c, e)| {
                    c.into_iter()
                        .zip(e.into_iter())
                        .map(|(c, e)| BilingualString::new(c, e))
                        .collect()
                })
                .unwrap_or_default(),
            update_time: chinese.update_time.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Warning {
    pub pieces: Vec<WarningPiece>,
}

impl Warning {
    pub fn new(chinese: Info, english: Info) -> Self {
        Self {
            pieces: chinese
                .details
                .zip(english.details)
                .map(|(c, e)| {
                    c.into_iter()
                        .zip(e.into_iter())
                        .map(|(c, e)| WarningPiece::new(c, e))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl_update!(warning);
