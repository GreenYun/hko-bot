// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use crate::{database::types::lang::Lang, statics};

pub fn to_string(lang: &Lang) -> String {
	lang.map(statics::SETLANG_MESSAGE_BILINGUAL, statics::SETLANG_MESSAGE_CHINESE, statics::SETLANG_MESSAGE_ENGLISH)
		.into()
}
