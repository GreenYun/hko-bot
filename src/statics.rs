// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-identifier: MIT

#![allow(dead_code)]

macro_rules! make_bilingual {
	($var:ident, $zh_str:literal, $en_str:literal) => {
		::paste::paste! {
			pub const [<$var _BILINGUAL>]: &str = concat!($zh_str, "\n", $en_str);
			pub const [<$var _CHINESE>]: &str = $zh_str;
			pub const [<$var _ENGLISH>]: &str = $en_str;
		}
	};
}

macro_rules! get_bilingual_str {
	($lang:expr, $var:ident) => {
		::paste::paste! {
			 $lang.map(
				$crate::statics::[<$var _BILINGUAL>],
				$crate::statics::[<$var _CHINESE>],
				$crate::statics::[<$var _ENGLISH>],
			 )
		}
	};
}

pub(crate) use get_bilingual_str;

// Server error messages

make_bilingual!(SERVER_ERROR_TIMEOUT, "連線逾時，請稍後再試。", "Connection timed out, please try again later.");

// Start messages

pub const START_MESSAGE_CHINESE: &str = r"歡迎新用户。

當前消息語言設定為<b>中文</b>，你可以隨時用 /setlang 變更設定。
有關命令嘅使用方式，請用 /help 查閲。";

pub const START_MESSAGE_ENGLISH: &str = r"Welcome new user.

You will receive messages in <b>English</b>, while you can change at any time with /setlang.
More about the commands, check /help.";

// Greetings

make_bilingual!(GREETINGS, "喂，老友。", "Hi, my old friend.");

// Help messages

pub const HELP_MESSAGE_BILINGUAL: &str = r"此機械人將提供來自香港天文台的天氣資訊。
This bot provides weather information from Hong Kong Observatory.

/help - 查看本幫助訊息 Look for help
/settings - 變更偏好設定 Modify preferences
/purge - 清除所有對話資料 Purge all current chat data
/briefing - 獲取本港地區天氣預報 Get local weather report
/bulletin - 獲取當前天氣報吿 Get current weather report
/warning - 獲取當前由天文台發出的天氣警報資料 Get the warning information from the Observatory";
pub const HELP_MESSAGE_CHINESE: &str = r"此機械人將提供來自香港天文台的天氣資訊。

/help - 查看本幫助訊息
/setlang - 變更偏好設定
/purge - 清除所有對話資料
/briefing - 獲取本港地區天氣預報
/bulletin - 獲取當前天氣報吿
/warning - 獲取當前由天文台發出的天氣警報資料";
pub const HELP_MESSAGE_ENGLISH: &str = r"This bot provides weather information from Hong Kong Observatory.

/help - Look for help
/settings - Modify preferences
/purge - Purge all current chat data
/briefing - Get local weather forecast
/bulletin - Get current weather report
/warning - Get the warning information from the Observatory";

// Settings messages

make_bilingual!(
	SETTINGS_MESSAGE_1,
	"呢度可以變更以下各項偏好設定。",
	"You can change your preference settings shown below."
);

make_bilingual!(
	SETTINGS_MESSAGE_2,
	"請點下面嘅掣以變更各項設定。",
	"Please click the following buttons to change each item."
);

pub const SETTINGS_MESSAGE_LANGUAGE_BILINGUAL: &str = r"語言 Language";

pub const SETTINGS_MESSAGE_LANGUAGE_CHINESE: &str = r"語言";

pub const SETTINGS_MESSAGE_LANGUAGE_ENGLISH: &str = r"Language";

// Setlang messages

make_bilingual!(
	SETLANG_MESSAGE,
	"而家開始我會用<b>中文</b>發消息畀你。",
	"You will receive <b>English</b> messages from now on."
);

// Setlang questions

pub const SETLANG_QUESTION_BILINGUAL: &str = r"你想我用何語言發消息？
What language do you want to receive messages in?";

// No warning messages

make_bilingual!(NO_WARNING_MESSAGE, "現時並無特別報告。", "There is no special announcement.");

// Briefing titles

make_bilingual!(BRIEFING_TITLE, "本港地區天氣預報", "Local Weather Forecast");

// Bulletin titles

make_bilingual!(BULLETIN_TITLE, "本港地區天氣報告", "Current Weather Report");

// Forecast titles

make_bilingual!(FORECAST_TITLE, "九天天氣預報", "9-day Weather Forecast");
