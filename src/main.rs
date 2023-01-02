// Copyright (c) 2022 GreenYun Organization
// SPDX-License-identifier: MIT

#[tokio::main]
async fn main() {
    println!(concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION")));

    let args = args::Args::new();

    config::logging();

    let db = database::connect(args.db_uri).await;
    let mut tg = telegram::connect(args.bot, db).await;

    tokio::join!(weather::update(), tg.dispatch());
}

mod args;
mod config;
mod database;
mod statics;
mod telegram;
mod tool;
mod weather;
