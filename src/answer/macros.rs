// Copyright (c) 2024 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

#[rustfmt::skip]
macro_rules! zh_num {
    (1)  => ("一");
    (2)  => ("二");
    (3)  => ("三");
    (4)  => ("四");
    (5)  => ("五");
    (6)  => ("六");
    (7)  => ("七");
    (8)  => ("八");
    (9)  => ("九");
    (10) => ("十");
    (11) => ("十一");
    (12) => ("十二");
    (13) => ("十三");
    (14) => ("十四");
    (15) => ("十五");
    (16) => ("十六");
    (17) => ("十七");
    (18) => ("十八");
    (19) => ("十九");
    (20) => ("二十");
    (21) => ("二十一");
    (22) => ("二十二");
    (23) => ("二十三");
    (24) => ("二十四");
    (25) => ("二十五");
    (26) => ("二十六");
    (27) => ("二十七");
    (28) => ("二十八");
    (29) => ("二十九");
    (30) => ("三十");
    (31) => ("三十一");

    (@gen_match $e:expr; $($n:tt)*) => {
        match $e {
            $( $n => zh_num!($n), )*
            _ => unreachable!(),
        }
    };

    ($e:expr) => {
        zh_num!(@gen_match $e; 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
    };
}

macro_rules! zh_weekday {
	($w:expr) => {{
		use chrono::Weekday::*;
		match $w {
			Mon => "星期一",
			Tue => "星期二",
			Wed => "星期三",
			Thu => "星期四",
			Fri => "星期五",
			Sat => "星期六",
			Sun => "星期日",
		}
	}};
}

pub(super) use zh_num;
pub(super) use zh_weekday;
