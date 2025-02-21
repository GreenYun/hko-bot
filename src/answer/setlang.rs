// Copyright (c) 2024 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use crate::{database::types::lang::Lang, statics::get_bilingual_str};

pub fn to_string(lang: &Lang) -> String {
	get_bilingual_str!(lang, SETLANG_MESSAGE).into()
}
