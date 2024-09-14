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

impl NonEmptyExt for Vec<String> {
	fn get_non_empty(self) -> Option<String> {
		self.first().and_then(|s| s.clone().get_non_empty())
	}
}
