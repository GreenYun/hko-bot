// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-identifier: MIT

// Start messages

pub const START_MESSAGE_CHINESE: &str = r"歡迎新用户。

當前消息語言設定為<b>中文</b>，你可以隨時用 /setlang 變更設定。
有關命令嘅使用方式，請用 /help 查閲。";

pub const START_MESSAGE_ENGLISH: &str = r"Welcome new user.

You will receive messages in <b>English</b>, while you can change at any time with /setlang.
More about the commands, check /help.";

// Greetings

pub const GREETINGS_CHINESE: &str = "喂，老友。";
pub const GREETINGS_ENGLISH: &str = "Hi, my old friend.";

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

pub const SETTINGS_MESSAGE_1_BILINGUAL: &str = r"呢度可以變更以下各項偏好設定。
You can change your preference settings shown below.";

pub const SETTINGS_MESSAGE_1_CHINESE: &str = r"呢度可以變更以下各項偏好設定。";

pub const SETTINGS_MESSAGE_1_ENGLISH: &str = r"You can change your preference settings shown below.";

pub const SETTINGS_MESSAGE_2_BILINGUAL: &str = r"請點下面嘅掣以變更各項設定。
Please click the following buttons to change each item.";

pub const SETTINGS_MESSAGE_2_CHINESE: &str = r"請點下面嘅掣以變更各項設定。";

pub const SETTINGS_MESSAGE_2_ENGLISH: &str = r"Please click the following buttons to change each item.";

pub const SETTINGS_MESSAGE_LANGUAGE_BILINGUAL: &str = r"語言 Language";

pub const SETTINGS_MESSAGE_LANGUAGE_CHINESE: &str = r"語言";

pub const SETTINGS_MESSAGE_LANGUAGE_ENGLISH: &str = r"Language";

// Setlang messages

pub const SETLANG_MESSAGE_BILINGUAL: &str = r"而家開始我會用<b>中英雙語</b>發消息畀你。
You will receive <b>Bilingual</b> messages from now on.";

pub const SETLANG_MESSAGE_CHINESE: &str = "而家開始我會用<b>中文</b>發消息畀你。";

pub const SETLANG_MESSAGE_ENGLISH: &str = "You will receive <b>English</b> messages from now on.";

// Setlang questions

pub const SETLANG_QUESTION_BILINGUAL: &str = r"你想我用何語言發消息？
What language do you want to receive messages in?";

// No warning messages

pub const NO_WARNING_MESSAGE_BILINGUAL: &str = r"現時並無特別報告。
There is no special announcement.";

pub const NO_WARNING_MESSAGE_CHINESE: &str = "現時並無特別報告。";

pub const NO_WARNING_MESSAGE_ENGLISH: &str = "There is no special announcement.";
