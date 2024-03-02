// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::io::Write;

use log::LevelFilter;

pub fn logging() {
    let level = match std::env::var_os("HKO_BOT_DEBUG").map_or(3, |s| s.to_string_lossy().parse::<u64>().unwrap_or(0)) {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let is_display_time = std::env::var_os("HKO_BOT_TIME").map_or(false, |s| {
        let s = s.to_string_lossy();
        !(s.is_empty() || s.to_ascii_lowercase() == "false" || s == "0")
    });

    env_logger::builder()
        .format(move |buf, record| {
            let timestamp = buf.timestamp();
            let level = record.level();
            let style = buf.default_level_style(level);

            if is_display_time {
                writeln!(buf, "{timestamp} {style}{level}{style:#} {}", record.args())
            } else {
                writeln!(buf, "{style}{level}{style:#} {}", record.args())
            }
        })
        .write_style(env_logger::fmt::WriteStyle::Auto)
        .filter_level(level)
        .init();
}
