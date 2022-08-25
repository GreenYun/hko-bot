// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

macro_rules! unwrap_or_execute {
    ($ex:expr, | $e:tt $(,)? | $r:expr $(,)?) => {
        match $ex {
            Ok(x) => x,
            Err($e) => $r,
        }
    };
    ($ex:expr, || $r:expr $(,)?) => {
        match $ex {
            Some(x) => x,
            None => $r,
        }
    };
    ($ex:expr, | | $r:expr $(,)?) => {
        unwrap_or_execute!($ex, || $r)
    };
}

pub(crate) use unwrap_or_execute;
