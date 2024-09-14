// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{dispatching::UpdateHandler, prelude::*, utils::command::BotCommands, RequestError};

use super::command::Command;

pub fn schema() -> UpdateHandler<RequestError> {
	use dptree::case;

	dptree::filter_map(move |callback: CallbackQuery| callback.data.and_then(|s| Command::parse(&s, "").ok()))
		.branch(case!(Command::SetLang(lang)).endpoint(setlang::setlang))
}

mod setlang;
