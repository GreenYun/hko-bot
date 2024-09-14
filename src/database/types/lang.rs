// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use std::fmt::Display;

use sqlx::Type;
use strum::EnumString;

// CREATE TYPE lang AS ENUM ('Bilingual', 'Chinese', 'English');
#[derive(Clone, EnumString, Eq, Type, PartialEq)]
#[sqlx(type_name = "lang")]
#[strum(ascii_case_insensitive)]
pub enum Lang {
	Bilingual,
	Chinese,
	English,
}

impl Lang {
	#[inline]
	pub fn map<T>(&self, bilingual: T, chinese: T, english: T) -> T {
		match self {
			Self::Bilingual => bilingual,
			Self::Chinese => chinese,
			Self::English => english,
		}
	}
}

impl Display for Lang {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.map("雙語 Bilingual", "中文", "English"))
	}
}
