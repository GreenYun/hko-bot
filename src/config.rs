// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::io::Write;

use env_logger::TimestampPrecision;
use log::LevelFilter;

pub fn logging() {
    let level = std::env::var_os("HKO_BOT_VERBOSITY")
        .map(|s| s.to_string_lossy().to_ascii_lowercase())
        .map_or(LevelFilter::Info, |s| match s.as_str() {
            "err" | "error" | "1" => LevelFilter::Error,
            "warn" | "warning" | "2" => LevelFilter::Warn,
            "info" | "3" => LevelFilter::Info,
            "debug" | "4" => LevelFilter::Debug,
            "trace" | "5" => LevelFilter::Trace,
            _ => LevelFilter::Off,
        });

    let is_display_time = std::env::var_os("HKO_BOT_TIME")
        .map(|s| s.to_string_lossy().to_ascii_lowercase())
        .is_some_and(|s| !s.is_empty() && !matches!(s.as_str(), "0" | "false" | "no"));

    let is_syslog_style = std::env::var_os("HKO_BOT_LOG_STYLE")
        .map(|s| s.to_string_lossy().to_ascii_lowercase())
        .is_some_and(|s| s == "syslog");

    let mut builder = env_logger::builder();

    if is_syslog_style {
        builder.format(|buf, record| {
            writeln!(
                buf,
                "<{}>{}: {}",
                match record.level() {
                    log::Level::Error => 3,
                    log::Level::Warn => 4,
                    log::Level::Info => 6,
                    log::Level::Debug | log::Level::Trace => 7,
                },
                record.target(),
                record.args()
            )
        });
    }

    builder
        .format_timestamp(is_display_time.then_some(TimestampPrecision::Seconds))
        .write_style(env_logger::fmt::WriteStyle::Auto)
        .filter_level(level)
        .init();
}
