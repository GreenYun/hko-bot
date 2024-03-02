// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use crate::{database::types::lang::Lang, statics};

pub fn to_string(lang: &Lang) -> String {
    match lang {
        Lang::Bilingual => statics::SETLANG_MESSAGE_BILINGUAL,
        Lang::Chinese => statics::SETLANG_MESSAGE_CHINESE,
        Lang::English => statics::SETLANG_MESSAGE_ENGLISH,
    }
    .into()
}
