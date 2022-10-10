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

        match lang {
            Lang::Bilingual => {
                if !item.chinese.is_empty() {
                    let _ = writeln!(result, "{item:x}");
                }
                if !item.english.is_empty() {
                    let _ = writeln!(result, "{item:e}");
                }
            }
            Lang::Chinese => {
                if item.chinese.is_empty() {
                    continue;
                }
                let _ = writeln!(result, "{}", item.chinese.trim());
            }
            Lang::English => {
                if item.english.is_empty() {
                    continue;
                }
                let _ = writeln!(result, "{item:e}");
            }
        }

        let _ = writeln!(result);
    }

    result
}
