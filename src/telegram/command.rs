// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::Me,
    utils::command::{BotCommands, ParseError},
    RequestError,
};

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase")]
pub enum Command {
    Briefing,
    Current,
    Help,
    Purge,

    #[command(parse_with = "parse_setlang")]
    SetLang(Option<String>),
    Settings,
    Start,
}

fn parse_setlang(input: String) -> Result<(Option<String>,), ParseError> {
    if input.is_empty() {
        Ok((None,))
    } else {
        Ok((Some(input),))
    }
}

pub fn schema() -> UpdateHandler<RequestError> {
    use dptree::case;

    macro_rules! command_endpoint {
        ($($variant:ident)::+, $endpoint:ident) => {
            case!($($variant)::+).endpoint($endpoint::$endpoint)
        };
        ($($variant:ident)::+ ($($param:ident),+ $(,)?), $endpoint:ident) => {
            case!($($variant)::+ ($($param),+)).endpoint($endpoint::$endpoint)
        };
    }

    dptree::entry().branch(
        dptree::filter_map(move |message: Message, me: Me| {
            let bot_name = me.user.username.unwrap_or_default();
            message.text().and_then(|text| Command::parse(text, bot_name).ok())
        })
        .branch(command_endpoint!(Command::Start, start))
        .branch(command_endpoint!(Command::Help, help))
        .branch(command_endpoint!(Command::Settings, settings))
        .branch(command_endpoint!(Command::Purge, purge))
        .branch(command_endpoint!(Command::SetLang(lang), setlang))
        .branch(command_endpoint!(Command::Briefing, briefing)),
    )
}

pub(super) use setlang::{setlang_ikb, setlang_internal};

mod briefing;
mod help;
mod purge;
mod setlang;
mod settings;
mod start;
