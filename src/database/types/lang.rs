// Copyright (c) 2022 GreenYun Organizaiton
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
            Lang::Bilingual => "雙語 Bilingual",
            Lang::Chinese => "中文",
            Lang::English => "English",
        })
    }
}
