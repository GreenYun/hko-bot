// Copyright (c) 2022 - 2023 GreenYun Organization
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

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bilingual => "雙語 Bilingual",
            Self::Chinese => "中文",
            Self::English => "English",
        })
    }
}
