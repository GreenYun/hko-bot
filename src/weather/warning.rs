// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use chrono::{DateTime, FixedOffset};
use hko::weather::{warning::info::InfoDetail, Info as Source};
use tokio::sync::RwLock;

use crate::tool::types::BilingualString;

use super::{WeatherData, WeatherDataUpdater};

#[derive(Clone, Default)]
pub struct Piece {
	pub name: BilingualString,
	pub contents: Vec<BilingualString>,
	pub update_time: DateTime<FixedOffset>,
}

impl Piece {
	pub fn new(zh: InfoDetail, en: InfoDetail) -> Self {
		let mut chinese_name = format!("{:o}", zh.code);
		let mut english_name = format!("{:e}", en.code);

		if let Some(code) = zh.subtype {
			chinese_name.push_str(&format!("\u{ff1a}{code:o}"));
			english_name.push_str(&format!(": {code:e}"));
		}

		Self {
			name: BilingualString::new(chinese_name, english_name),
			contents: zh
				.contents
				.zip(en.contents)
				.map(|(c, e)| {
					let mut c = c.into_iter().collect::<Vec<_>>();
					let mut e = e.into_iter().collect::<Vec<_>>();

					let c_len = c.len();
					let e_len = e.len();

					match c_len.cmp(&e_len) {
						std::cmp::Ordering::Less => {
							let mut v = vec![String::new(); e_len - c_len];
							v.append(&mut c);
							c = v;
						}
						std::cmp::Ordering::Greater => {
							let mut v = vec![String::new(); c_len - e_len];
							v.append(&mut e);
							e = v;
						}
						std::cmp::Ordering::Equal => {}
					}

					c.into_iter().zip(e).map(|(c, e)| BilingualString::new(c, e)).collect()
				})
				.unwrap_or_default(),
			update_time: zh.update_time.unwrap_or_default(),
		}
	}
}

#[derive(Clone, Default)]
pub struct Warning {
	pub pieces: Vec<Piece>,
}

static STORE: OnceLock<RwLock<Warning>> = OnceLock::new();

impl Warning {
	fn new(zh: Source, en: Source) -> Self {
		Self {
			pieces: zh
				.details
				.zip(en.details)
				.map(|(c, e)| c.into_iter().zip(e).map(|(c, e)| Piece::new(c, e)).collect())
				.unwrap_or_default(),
		}
	}
}

impl From<(Source, Source)> for Warning {
	fn from((zh, en): (Source, Source)) -> Self {
		Self::new(zh, en)
	}
}

impl WeatherData for Warning {
	fn get_store() -> &'static OnceLock<RwLock<Self>> {
		&STORE
	}
}

impl WeatherDataUpdater for Warning {
	type Source = Source;
}
