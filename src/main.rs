// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

#[tokio::main]
async fn main() {
    let args = args::Args::new();

    config::logging();

    let db = database::connect(args.db_uri).await;
    let mut tg = telegram::connect(args.bot, db).await;

    tg.dispatch().await;
}

mod args;
mod config;
mod database;
mod macros;
mod statics;
mod telegram;
