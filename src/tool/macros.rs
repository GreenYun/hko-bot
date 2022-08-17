// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

macro_rules! unwrap_or_excute {
    ($ex:expr, || $r:expr $(,)?) => {
        match { $ex } {
            Some(x) => x,
            None => $r,
        }
    };
    ($ex:expr, | | $r:expr $(,)?) => {
        unwrap_or_excute!($ex, || $r)
    };
    ($ex:expr, | $e:tt $(,)? | $r:expr $(,)?) => {
        match { $ex } {
            Ok(x) => x,
            Err($e) => $r,
        }
    };
}

pub(crate) use unwrap_or_excute;
