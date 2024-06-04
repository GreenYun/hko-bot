// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

#![allow(clippy::module_name_repetitions)]

pub trait NonEmptyExt {
    fn get_non_empty(self) -> Option<String>;
}

impl<T> NonEmptyExt for T
where
    T: Into<String>,
{
    fn get_non_empty(self) -> Option<String> {
        Some(self.into()).filter(|s| !s.is_empty())
    }
}
