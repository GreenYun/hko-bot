// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use chrono::{DateTime, FixedOffset};

#[derive(Clone, Default)]
struct AnswerStore {
	inner: Vec<String>,
	update_time: DateTime<FixedOffset>,
}

impl AnswerStore {
	const fn new(inner: Vec<String>, update_time: DateTime<FixedOffset>) -> Self {
		Self { inner, update_time }
	}
}

pub trait Answer {
	async fn answer(lang: &Lang) -> Vec<String>;
}

pub use briefing::Briefing;
pub use bulletin::Bulletin;
pub use setlang::to_string as setlang;

use crate::database::types::lang::Lang;

mod briefing;
mod bulletin;
mod setlang;
