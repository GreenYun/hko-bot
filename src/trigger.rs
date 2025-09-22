// Copyright (c) 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{
	collections::HashMap,
	fmt::Write,
	sync::{LazyLock, OnceLock},
};

use chrono::{DateTime, Utc};
use teloxide::{prelude::*, types::ParseMode};
use tokio::sync::RwLock;

use crate::{
	database::types::lang::Lang,
	tool::mix_strings,
	weather::{WeatherData, warning as weather_warning},
};

static CHANNEL_CHAT_ID: ChatId = ChatId(-1_001_692_976_401);
static BOT: OnceLock<Bot> = OnceLock::new();
static LAST_UPDATE: LazyLock<RwLock<HashMap<String, DateTime<Utc>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn set_bot(bot: Bot) {
	let _ = BOT.set(bot);
}

pub async fn trigger() {
	let Some(bot) = BOT.get() else {
		return;
	};

	let Some(warning) = weather_warning::Warning::get().await else {
		return;
	};

	for p in &warning.pieces {
		let update_time = p.update_time.to_utc();
		let name = &p.name.en;

		let need_send = {
			let last_update = LAST_UPDATE.read().await;
			last_update.get(name).is_none_or(|last| &update_time > last)
		};

		if need_send {
			{
				LAST_UPDATE.write().await.insert(name.clone(), update_time);
			}

			let chinese = piece_to_string(p, &Lang::Chinese);
			let english = piece_to_string(p, &Lang::English);

			bot.send_message(CHANNEL_CHAT_ID, chinese).parse_mode(ParseMode::Html).await.ok();
			bot.send_message(CHANNEL_CHAT_ID, english).parse_mode(ParseMode::Html).await.ok();
		}
	}
}

fn piece_to_string(p: &weather_warning::Piece, lang: &Lang) -> String {
	let mut list = vec!["<b>".to_string() + p.name.clone() + "</b>"];
	list.extend_from_slice(&p.contents);

	let mut text = mix_strings(lang, &list);

	write!(text, "\n\n<i>@ {}</i>", p.update_time).ok();

	text
}
