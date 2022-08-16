// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

macro_rules! unwrap_or_excute {
    ($ex:expr, None | | $r:expr $(,)?) => {
        match { $ex } {
            Some(x) => x,
            None => $r,
        }
    };
    ($ex:expr, Err | $e:tt | $r:expr $(,)?) => {
        match { $ex } {
            Ok(x) => x,
            Err($e) => $r,
        }
    };
}

pub(crate) use unwrap_or_excute;
