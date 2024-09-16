// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use std::{
	fmt::{LowerExp, LowerHex},
	ops::Add,
};

#[allow(clippy::module_name_repetitions)]
pub struct BilingualStr<'a> {
	pub chinese: &'a str,
	pub english: &'a str,
}

impl<'a> BilingualStr<'a> {
	pub const fn new(chinese: &'a str, english: &'a str) -> Self {
		Self { chinese, english }
	}

	pub const fn is_empty(&self) -> bool {
		self.chinese.is_empty() && self.english.is_empty()
	}
}

impl<'a> LowerExp for BilingualStr<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.english)
	}
}

impl<'a> LowerHex for BilingualStr<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.chinese)
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct BilingualString {
	pub chinese: String,
	pub english: String,
}

impl BilingualString {
	pub fn new<S1, S2>(chinese: S1, english: S2) -> Self
	where
		S1: Into<String>,
		S2: Into<String>,
	{
		Self { chinese: chinese.into(), english: english.into() }
	}

	pub fn as_str(&self) -> BilingualStr {
		BilingualStr { chinese: self.chinese.as_str(), english: self.english.as_str() }
	}

	pub fn is_empty(&self) -> bool {
		self.chinese.is_empty() && self.english.is_empty()
	}

	#[allow(clippy::missing_const_for_fn)]
	pub fn unzip(self) -> (String, String) {
		(self.chinese, self.english)
	}

	pub fn add_single_newline(self) -> Self {
		if self.chinese.is_empty() {
			return self;
		}

		Self { chinese: self.chinese.trim().to_string() + "\n", english: self.english }
	}
}

impl Add for BilingualString {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self { chinese: self.chinese + other.chinese.as_str(), english: self.english + other.english.as_str() }
	}
}

impl Add<&str> for BilingualString {
	type Output = Self;

	fn add(self, other: &str) -> Self {
		Self { chinese: self.chinese + other, english: self.english + other }
	}
}

impl Add<BilingualString> for String {
	type Output = BilingualString;

	fn add(self, other: BilingualString) -> BilingualString {
		BilingualString { chinese: self.clone() + other.chinese.as_str(), english: self + other.english.as_str() }
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
