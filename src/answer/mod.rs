// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use chrono::{DateTime, FixedOffset};

#[derive(Clone, Default)]
struct AnswerStore {
    inner: String,
    update_time: DateTime<FixedOffset>,
}

pub trait Answer {
    async fn answer(lang: &Lang) -> String;
}

pub use briefing::Briefing;
pub use bulletin::to_string as bulletin;
pub use setlang::to_string as setlang;

use crate::database::types::lang::Lang;

mod briefing;
mod bulletin;
mod setlang;
