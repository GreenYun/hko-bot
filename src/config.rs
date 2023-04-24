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

    let is_display_time = match std::env::var_os("HKO_BOT_TIME").map(|s| {
        let s = s.to_string_lossy();
        s.is_empty() || s.to_ascii_lowercase() == "false" || s == "0"
    }) {
        Some(true) => true,
        None | Some(false) => false,
    };

    let is_journal = !atty::is(atty::Stream::Stdout);

    env_logger::builder()
        .format(move |buf, record| {
            use env_logger::fmt::Color;
            use log::Level;

            if is_journal {
                let level = 3 * 8
                    + match record.level() {
                        Level::Error => 3,
                        Level::Warn => 4,
                        Level::Info => 5,
                        Level::Debug => 6,
                        Level::Trace => 7,
                    };

                return writeln!(buf, "<{level}> {}", record.args());
            }

            let mut style = buf.style();
            let timestamp = style
                .set_color(Color::Rgb(128, 128, 128))
                .value(format!("[{}]", buf.timestamp()));

            let mut style = buf.style();
            let level = match record.level() {
                Level::Error => style.set_color(Color::Red).value("ERROR"),
                Level::Warn => style.set_color(Color::Yellow).value("WARN "),
                Level::Info => style.set_color(Color::Green).value("INFO "),
                Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
                Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
            };

            if is_display_time {
                writeln!(buf, "\r{timestamp} {level} {}", record.args())
            } else {
                writeln!(buf, "\r{level} {}", record.args())
            }
        })
        .write_style(env_logger::fmt::WriteStyle::Auto)
        .filter_level(level)
        .init();
}
