// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::io::Write;

use log::LevelFilter;

// use crate::config::internal::TTYSetColor;

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
        s.is_empty() || s.to_ascii_lowercase() == "false" || s == "0"
    });

    env_logger::builder()
        .format(move |buf, record| {
            let timestamp = buf.timestamp();

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

// mod internal {
//     use env_logger::fmt::{Color, Style};

//     pub trait TTYSetColor {
//         fn tty_set_color(&mut self, color: Color) -> &mut Self;
//     }

//     impl TTYSetColor for Style {
//         fn tty_set_color(&mut self, color: Color) -> &mut Self {
//             if atty::is(atty::Stream::Stdout) {
//                 self.set_color(color)
//             } else {
//                 self
//             }
//         }
//     }
// }
