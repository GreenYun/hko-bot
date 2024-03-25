// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

pub const NAME_VERSION_STRING: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() {
    let args = args::Args::new();

    config::logging();
    log::info!("{NAME_VERSION_STRING}");

    let db = database::connect(args.db_uri).await;
    let mut tg = telegram::connect(args.bot, db).await;

    tokio::join!(weather::update(), tg.dispatch());
}

mod answer;
mod args;
mod config;
mod database;
mod statics;
mod telegram;
mod tool;
mod weather;
