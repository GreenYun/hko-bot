// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

#[tokio::main]
async fn main() {
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
