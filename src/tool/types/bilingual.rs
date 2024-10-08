// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use std::{
	fmt::{LowerExp, LowerHex},
	ops::Add,
};

#[allow(clippy::module_name_repetitions)]
pub struct BilingualStr<'a> {
	pub zh: &'a str,
	pub en: &'a str,
}

impl<'a> BilingualStr<'a> {
	pub const fn new(chinese: &'a str, english: &'a str) -> Self {
		Self { zh: chinese, en: english }
	}

	pub const fn is_empty(&self) -> bool {
		self.zh.is_empty() && self.en.is_empty()
	}
}

impl LowerExp for BilingualStr<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.en)
	}
}

impl LowerHex for BilingualStr<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.zh)
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct BilingualString {
	pub zh: String,
	pub en: String,
}

impl BilingualString {
	pub fn new<S1, S2>(zh: S1, en: S2) -> Self
	where
		S1: Into<String>,
		S2: Into<String>,
	{
		Self { zh: zh.into(), en: en.into() }
	}

	pub fn as_str(&self) -> BilingualStr {
		BilingualStr { zh: self.zh.as_str(), en: self.en.as_str() }
	}

	pub fn is_empty(&self) -> bool {
		self.zh.is_empty() && self.en.is_empty()
	}

	#[allow(clippy::missing_const_for_fn)]
	pub fn unzip(self) -> (String, String) {
		(self.zh, self.en)
	}

	pub fn add_single_newline(self) -> Self {
		if self.zh.is_empty() {
			return self;
		}

		Self { zh: self.zh.trim().to_string() + "\n", en: self.en }
	}
}

impl Add for BilingualString {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self { zh: self.zh + other.zh.as_str(), en: self.en + other.en.as_str() }
	}
}

impl Add<&str> for BilingualString {
	type Output = Self;

	fn add(self, other: &str) -> Self {
		Self { zh: self.zh + other, en: self.en + other }
	}
}

impl Add<BilingualString> for String {
	type Output = BilingualString;

	fn add(self, other: BilingualString) -> BilingualString {
		BilingualString { zh: self.clone() + other.zh.as_str(), en: self + other.en.as_str() }
	}
}

impl LowerExp for BilingualString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.en)
	}
}

impl LowerHex for BilingualString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.zh)
	}
}
