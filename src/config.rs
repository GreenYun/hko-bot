// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

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

    env_logger::builder()
        .format_timestamp(is_display_time.then_some(TimestampPrecision::Seconds))
        .write_style(env_logger::fmt::WriteStyle::Auto)
        .filter_level(level)
        .init();
}
