// Copyright (c) 2022 GreenYun Organization
// SPDX-License-identifier: MIT

use std::{
    fmt::{LowerExp, LowerHex},
    ops::Add,
};

#[derive(Clone, Default)]
pub struct BilingualString {
    pub chinese: String,
    pub english: String,
}

impl BilingualString {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<S>(chinese: S, english: S) -> Self
    where
        S: ToString,
    {
        Self {
            chinese: chinese.to_string(),
            english: english.to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chinese.is_empty() && self.english.is_empty()
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn unzip(self) -> (String, String) {
        (self.chinese, self.english)
    }

    pub fn add_single_newline(self) -> Self {
        if self.is_empty() {
            return self;
        }

        Self {
            chinese: self.chinese + "\n",
            english: self.english,
        }
    }
}

impl Add for BilingualString {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            chinese: self.chinese + other.chinese.as_str(),
            english: self.english + other.english.as_str(),
        }
    }
}

impl Add<&str> for BilingualString {
    type Output = Self;

    fn add(self, other: &str) -> Self {
        Self {
            chinese: self.chinese + other,
            english: self.english + other,
        }
    }
}

impl Add<BilingualString> for String {
    type Output = BilingualString;

    fn add(self, other: BilingualString) -> BilingualString {
        BilingualString {
            chinese: self.clone() + other.chinese.as_str(),
            english: self + other.english.as_str(),
        }
    }
}

impl LowerExp for BilingualString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.english)
    }
}

impl LowerHex for BilingualString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.chinese)
    }
}
