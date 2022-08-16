// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::io::Write;

use log::LevelFilter;

pub fn logging() {
    let level = match std::env::var_os("HKO_BOT_DEBUG")
        .map(|s| s.to_string_lossy().parse::<u64>().unwrap_or(0))
        .unwrap_or(3)
    {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::builder()
        .format(|buf, record| {
            use env_logger::fmt::Color;
            use log::Level;

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

            writeln!(buf, "\r{} {} {}", timestamp, level, record.args())
        })
        .write_style(env_logger::fmt::WriteStyle::Auto)
        .filter_level(level)
        .init();
}
