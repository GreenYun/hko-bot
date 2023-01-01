// Copyright (c) 2022 GreenYun Organization
// SPDX-License-identifier: MIT

use chrono::{DateTime, FixedOffset};
use hko::weather::{warning::info::InfoDetail, Info};

use crate::tool::{macros::unwrap_or_execute, types::BilingualString};

use super::macros::impl_update;

#[derive(Clone, Default)]
pub struct Piece {
    pub name: BilingualString,
    pub contents: Vec<BilingualString>,
    pub update_time: DateTime<FixedOffset>,
}

impl Piece {
    pub fn new(chinese: InfoDetail, english: InfoDetail) -> Self {
        let mut chinese_name = format!("{:o}", chinese.code);
        let mut english_name = format!("{:e}", english.code);

        if let Some(code) = chinese.subtype {
            chinese_name.push_str(&format!("：{code:o}"));
            english_name.push_str(&format!(": {code:e}"));
        }

        Self {
            name: BilingualString::new(chinese_name, english_name),
            contents: chinese
                .contents
                .zip(english.contents)
                .map(|(c, e)| {
                    let mut c = c.into_iter().collect::<Vec<_>>();
                    let mut e = e.into_iter().collect::<Vec<_>>();

                    let c_len = c.len();
                    let e_len = e.len();

                    match c_len.cmp(&e_len) {
                        std::cmp::Ordering::Less => {
                            let mut v = vec![String::new(); e_len - c_len];
                            v.extend_from_slice(&c);
                            c = v;
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            let mut v = vec![String::new(); c_len - e_len];
                            v.extend_from_slice(&e);
                            e = v;
                        }
                    }

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
    pub pieces: Vec<Piece>,
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
                        .map(|(c, e)| Piece::new(c, e))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl_update!(warning);
