// Copyright (c) 2022 GreenYun Organization
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
                    writeln!(result, "{item:x}").ok();
                }
                if !item.english.is_empty() {
                    writeln!(result, "{item:e}").ok();
                }
            }
            Lang::Chinese => {
                if item.chinese.is_empty() {
                    continue;
                }
                writeln!(result, "{}", item.chinese.trim()).ok();
            }
            Lang::English => {
                if item.english.is_empty() {
                    continue;
                }
                writeln!(result, "{item:e}").ok();
            }
        }

        writeln!(result).ok();
    }

    result
}
