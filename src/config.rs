// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::io::Write;

use env_logger::TimestampPrecision;
use log::LevelFilter;

fn get_lowercase_env_var(key: &str) -> Option<String> {
    std::env::var_os(key).map(|s| s.to_string_lossy().to_ascii_lowercase())
}

pub fn logging() {
    let level = get_lowercase_env_var("HKO_BOT_LOG_LEVEL").map_or(LevelFilter::Info, |s| match s.as_str() {
        "err" | "error" | "1" => LevelFilter::Error,
        "warn" | "warning" | "2" => LevelFilter::Warn,
        "info" | "3" => LevelFilter::Info,
        "debug" | "4" => LevelFilter::Debug,
        "trace" | "5" => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });

    let is_display_time = get_lowercase_env_var("HKO_BOT_LOG_TIME")
        .is_some_and(|s| !s.is_empty() && !matches!(s.as_str(), "0" | "false" | "no"));

    let is_syslog_style = get_lowercase_env_var("HKO_BOT_LOG_STYLE").is_some_and(|s| s == "syslog");

    let mut builder = env_logger::builder();

    if is_syslog_style {
        builder.format(|buf, record| {
            use log::Level;

            let syslog_level = match record.level() {
                Level::Error => 3,
                Level::Warn => 4,
                Level::Info => 6,
                Level::Debug | Level::Trace => 7,
            };
            writeln!(buf, "<{syslog_level}>[{}] {}", record.target(), record.args())
        });
    }

    builder
        .format_timestamp(is_display_time.then_some(TimestampPrecision::Seconds))
        .write_style(env_logger::fmt::WriteStyle::Auto)
        .filter_level(level)
        .init();
}
