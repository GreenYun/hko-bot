// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

macro_rules! count_tt {
    () => {
        0
    };
    ($x:tt $($y:tt)*) => {
        1 + $crate::weather::macros::count_tt!($($y)*)
    };
}

macro_rules! weather_mods {
    {
        $(mod $x:ident;)+
        const $all_updaters:ident: [&$updater_type:ident; $count:ident];
    } => {
        $(
            mod $x;
        )+

        $(::paste::paste! {
            ::lazy_static::lazy_static! {
                static ref [<__ $x:upper>]: ::std::sync::Arc<::tokio::sync::RwLock<[<$x:lower>]::[<$x:camel>]>> =
                    ::std::sync::Arc::new(::tokio::sync::RwLock::new(Default::default()));
            }

            #[inline]
            pub fn [<$x:lower>]() -> ::std::sync::Arc<::tokio::sync::RwLock<[<$x:lower>]::[<$x:camel>]>> {
                [<__ $x:upper>].clone()
            }
        })+

        type $updater_type = fn() -> ::tokio::task::JoinHandle<()>;

        #[allow(non_upper_case_globals)]
        const $count: usize = $crate::weather::macros::count_tt!($($x)+);

        #[allow(non_upper_case_globals)]
        const $all_updaters: [&$updater_type; $count] = {
            ::paste::paste! {
                $(
                    const [<$x:lower _ updater>]: $updater_type = || {
                        ::tokio::spawn(<[<$x:lower>]::[<$x:camel>] as $crate::weather::AsyncUpdater>::update())
                    };
                )+

                [$(&[<$x:lower _ updater>]),+]
            }
        };
    };
}

pub(super) use count_tt;
pub(super) use weather_mods;
