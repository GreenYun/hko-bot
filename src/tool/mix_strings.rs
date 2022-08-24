// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::fmt::Write;

use crate::database::types::lang::Lang;

use super::types::BilingualString;

pub fn mix_strings(list: Vec<BilingualString>, lang: &Lang) -> String {
    let mut result = String::new();

    for item in list {
        if item.is_empty() {
            continue;
        }

        if matches!(lang, Lang::Chinese) {
            let _ = writeln!(result, "{}", item.chinese.trim());
        };
        if matches!(lang, Lang::Bilingual) {
            let _ = writeln!(result, "{:x}", item);
        };
        if !matches!(lang, Lang::Chinese) {
            let _ = writeln!(result, "{:e}", item);
        }

        let _ = writeln!(result);
    }

    result
}
