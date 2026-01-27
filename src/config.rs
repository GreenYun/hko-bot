// Copyright (c) 2022 - 2026 GreenYun Organization
// SPDX-License-Identifier: MIT

use env_logger::{TimestampPrecision, WriteStyle};
use log::LevelFilter;
use syslog::Facility;

fn get_lowercase_env_var(key: &str) -> Option<String> {
	std::env::var_os(key).map(|s| s.to_string_lossy().to_ascii_lowercase())
}

pub fn logger_init() {
	let level = get_lowercase_env_var("HKO_BOT_LOG_LEVEL").map_or(LevelFilter::Info, |s| match s.as_str() {
		"err" | "error" | "1" => LevelFilter::Error,
		"warn" | "warning" | "2" => LevelFilter::Warn,
		"info" | "3" => LevelFilter::Info,
		"debug" | "4" => LevelFilter::Debug,
		"trace" | "5" => LevelFilter::Trace,
		_ => LevelFilter::Off,
	});

	let is_syslog_style = get_lowercase_env_var("HKO_BOT_LOG_STYLE").is_some_and(|s| s == "syslog");

	if is_syslog_style && syslog_init(level).is_ok() {
		return;
	}

	let show_time = get_lowercase_env_var("HKO_BOT_LOG_TIME")
		.is_some_and(|s| !s.is_empty() && !matches!(s.as_str(), "0" | "false" | "no"));
	env_logger_init(show_time, level);
}

fn syslog_init(level: LevelFilter) -> syslog::Result<()> {
	syslog::init_unix(Facility::LOG_LOCAL7, level)
}

fn env_logger_init(show_time: bool, level: LevelFilter) {
	let mut builder = env_logger::builder();

	builder
		.format_timestamp(show_time.then_some(TimestampPrecision::Seconds))
		.write_style(WriteStyle::Auto)
		.filter_level(level)
		.init();
}
