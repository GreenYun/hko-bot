// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::future::Future;

use chrono::{DateTime, FixedOffset};
use tokio::sync::RwLock;

use crate::database::types::lang::Lang;

pub use briefing::Briefing;
pub use bulletin::Bulletin;
pub use warning::Warning;

pub use setlang::to_string as setlang;

pub trait Answer {
	async fn answer(lang: &Lang) -> Vec<String>;
}

#[derive(Clone, Default)]
struct AnswerEntry {
	inner: Vec<String>,
	update_time: DateTime<FixedOffset>,
}

impl AnswerEntry {
	const fn new(inner: Vec<String>, update_time: DateTime<FixedOffset>) -> Self {
		Self { inner, update_time }
	}

	fn new_err(err: &str) -> Self {
		Self::new(vec![err.to_string()], DateTime::default())
	}
}

#[derive(Default)]
struct AnswerStore {
	bi: RwLock<AnswerEntry>,
	en: RwLock<AnswerEntry>,
	zh: RwLock<AnswerEntry>,
}

impl AnswerStore {
	async fn get(&self, lang: &Lang) -> AnswerEntry {
		let lock = lang.map(&self.bi, &self.zh, &self.en);
		let ent = lock.read().await;
		ent.clone()
	}

	async fn update_and_get<'a, F, Fut>(&self, lang: &'a Lang, update: F) -> AnswerEntry
	where
		F: FnOnce(&'a Lang, AnswerEntry) -> Fut + Send,
		Fut: Future<Output = AnswerEntry> + Send,
	{
		let ent = self.get(lang).await;
		let old_time = ent.update_time;

		let ent = update(lang, ent).await;

		if ent.update_time > old_time {
			self.set(lang, ent.clone()).await;
		}

		ent
	}

	async fn set(&self, lang: &Lang, entry: AnswerEntry) {
		let lock = lang.map(&self.bi, &self.zh, &self.en);
		let mut ent = lock.write().await;
		*ent = entry;
	}
}

mod briefing;
mod bulletin;
mod setlang;
mod warning;
