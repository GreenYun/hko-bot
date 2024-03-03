// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(clippy::module_name_repetitions)]

pub trait NonEmptyExt {
    fn get_non_empty(self) -> Option<String>;
}

impl NonEmptyExt for String {
    fn get_non_empty(self) -> Option<String> {
        Some(self).filter(|s| !s.is_empty())
    }
}

impl NonEmptyExt for &str {
    fn get_non_empty(self) -> Option<String> {
        Some(self.to_string()).filter(|s| !s.is_empty())
    }
}

impl NonEmptyExt for Option<String> {
    fn get_non_empty(self) -> Option<String> {
        self.filter(|s| !s.is_empty())
    }
}
