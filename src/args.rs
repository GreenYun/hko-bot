// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{env, io, process};

use crate::NAME_VERSION_STRING;

#[derive(Clone)]
pub struct Args {
    pub bot: String,
    pub db_uri: String,
}

impl Args {
    pub fn new() -> Self {
        let mut bot = String::new();
        let mut db_uri = String::new();

        env::vars_os()
            .map(|var| {
                (
                    var.0.to_string_lossy().into_owned().to_uppercase(),
                    var.1.to_string_lossy().into_owned(),
                )
            })
            .for_each(|(key, val)| match key.as_str() {
                "HKO_BOT_TOKEN" => bot = val,
                "HKO_BOT_DATABASE_URI" => db_uri = val,
                _ => (),
            });

        let mut args_os = env::args_os().skip(1);
        while let Some(arg) = args_os.next() {
            let arg = arg.to_string_lossy();
            match arg.as_ref() {
                "--help" => usage_then_exit(0),
                "--version" => {
                    println!("{NAME_VERSION_STRING}");
                    process::exit(0);
                }

                // Bot token
                "-k" => {
                    bot = args_os
                        .next()
                        .unwrap_or_else(|| usage_then_exit(1))
                        .to_string_lossy()
                        .into_owned();
                }

                // Database URI
                "-s" => {
                    db_uri = args_os
                        .next()
                        .unwrap_or_else(|| usage_then_exit(1))
                        .to_string_lossy()
                        .into_owned();
                }

                _ => usage_then_exit(1),
            }
        }

        if bot.is_empty() {
            eprintln!("error: bot token is required");
            usage_then_exit(1);
        }

        if db_uri.is_empty() {
            eprintln!("error: database URI is required");
            usage_then_exit(1);
        }

        Self { bot, db_uri }
    }
}

fn program_call_name() -> String {
    env::args_os()
        .next()
        .unwrap_or_default()
        .to_string_lossy()
        .split('/')
        .last()
        .unwrap()
        .into()
}

fn usage(mut w: impl io::Write) {
    writeln!(w, "usage: {} [-k bot_token] [-s database_uri]", program_call_name()).ok();
}

fn usage_then_exit(exit_val: i32) -> ! {
    if exit_val == 0 {
        usage(io::stdout());
    } else {
        usage(io::stderr());
    }

    process::exit(exit_val)
}
