// Copyright (c) 2022 - 2025 GreenYun Organization
// SPDX-License-identifier: MIT

use std::fmt::Write;

use crate::database::types::lang::Lang;

use super::types::{BilingualStr, BilingualString};

pub fn mix_string(lang: &Lang, s: &BilingualStr) -> String {
	if s.is_empty() {
		return String::new();
	}

	let lang = if matches!(lang, Lang::Bilingual) {
		match (s.zh.is_empty(), s.en.is_empty()) {
			(true, _) => &Lang::English,
			(false, true) => &Lang::Chinese,
			(false, false) => &Lang::Bilingual,
		}
	} else {
		lang
	};

	match lang {
		Lang::Bilingual => format!("{s:x}\n{s:e}").trim().to_string(),
		Lang::Chinese => s.zh.trim().to_string(),
		Lang::English => s.en.trim().to_string(),
	}
}

pub fn mix_strings(lang: &Lang, list: &[BilingualString]) -> String {
	let mut result = String::new();

	for item in list {
		if item.is_empty() {
			continue;
		}

		let mixed = mix_string(lang, &item.as_str());
		if mixed.is_empty() {
			continue;
		}

		if !result.is_empty() {
			write!(result, "\n\n").ok();
		}

		write!(result, "{mixed}").ok();
	}

	result
}

#[cfg(test)]
mod test {
	#[test]
	fn test() {
		use super::{BilingualStr, BilingualString, mix_string, mix_strings};
		use crate::database::types::lang::Lang;

		let str1 = BilingualStr::new("中文", "Chinese");
		let str2 = BilingualStr::new("", "English");
		let str3 = BilingualStr::new("中文", "");
		let str4 = BilingualStr::new("", "");

		assert_eq!(mix_string(&Lang::Bilingual, &str1), "中文\nChinese");
		assert_eq!(mix_string(&Lang::Bilingual, &str2), "English");
		assert_eq!(mix_string(&Lang::Bilingual, &str3), "中文");
		assert_eq!(mix_string(&Lang::Bilingual, &str4), "");

		assert_eq!(mix_string(&Lang::Chinese, &str1), "中文");
		assert_eq!(mix_string(&Lang::Chinese, &str2), "");
		assert_eq!(mix_string(&Lang::Chinese, &str3), "中文");
		assert_eq!(mix_string(&Lang::Chinese, &str4), "");

		assert_eq!(mix_string(&Lang::English, &str1), "Chinese");
		assert_eq!(mix_string(&Lang::English, &str2), "English");
		assert_eq!(mix_string(&Lang::English, &str3), "");
		assert_eq!(mix_string(&Lang::English, &str4), "");

		let list = vec![
			BilingualString::new("中文1", "English1"),
			BilingualString::new("", "English2"),
			BilingualString::new("中文3", ""),
			BilingualString::new("", ""),
		];

		assert_eq!(mix_strings(&Lang::Bilingual, &list), "中文1\nEnglish1\n\nEnglish2\n\n中文3");
		assert_eq!(mix_strings(&Lang::Chinese, &list), "中文1\n\n中文3");
		assert_eq!(mix_strings(&Lang::English, &list), "English1\n\nEnglish2");

		let list = vec![
			BilingualString::new("中文1", "English1").add_single_newline(),
			BilingualString::new("", "English2").add_single_newline(),
			BilingualString::new("中文3", "").add_single_newline(),
			BilingualString::new("", "").add_single_newline(),
		];

		assert_eq!(mix_strings(&Lang::Bilingual, &list), "中文1\n\nEnglish1\n\nEnglish2\n\n中文3");
		assert_eq!(mix_strings(&Lang::Chinese, &list), "中文1\n\n中文3");
		assert_eq!(mix_strings(&Lang::English, &list), "English1\n\nEnglish2");
	}
}
