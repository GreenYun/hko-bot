// Copyright (c) 2022 GreenYun Organization
// SPDX-License-identifier: MIT

macro_rules! command_endpoint {
    (@parse $($enum_path:path)?, $variant:ident,) => {
        ::paste::paste! {
            ::teloxide::dptree::case!($($enum_path ::)? [<$variant:camel>])
                .endpoint([<$variant:lower>] :: [<$variant:lower>])
        }
    };
    (@parse $($enum_path:path)?, $variant:ident, $($rem:ident,)+) => {
        $crate::telegram::command::macros::command_endpoint!(@parse $($enum_path ::)? $variant, $($rem,)+)
    };
    (@parse $variant:ident,) => {
        $crate::telegram::command::macros::command_endpoint!(@parse , $variant,)
    };
    ($($enum_path:ident)::+) => {
        $crate::telegram::command::macros::command_endpoint!(@parse $($enum_path,)+)
    };

    (@parse $($enum_path:path)?, $variant:ident, ($($param:ident),+)) => {
        ::paste::paste! {
            ::teloxide::dptree::case!($($enum_path ::)? [< $variant:camel >] ($($param),+))
                .endpoint([< $variant:lower >] :: [< $variant:lower >])
        }
    };
    (@parse $($enum_path:path)?, $variant:ident, $($rem:ident,)+ ($($param:ident),+)) => {
        $crate::telegram::command::macros::command_endpoint!(@parse $enum_path :: $variant, $($rem,)+ ($($param),+))
    };
    (@parse $variant:ident, ($($param:ident),+)) => {
        $crate::telegram::command::macros::command_endpoint!(@parse , $variant, ($($param),+))
    };
    ($($enum_path:ident)::+ ($($param:ident),+ $(,)?)) => {
        $crate::telegram::command::macros::command_endpoint!(@parse $($enum_path,)+ ($($param),+))
    };
}

pub(super) use command_endpoint;
