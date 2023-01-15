// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-Identifier: MIT

macro_rules! count_tt {
    () => {
        0
    };
    ($x:tt $($y:tt)*) => {
        1 + $crate::weather::macros::count_tt!($($y)*)
    };
}

macro_rules! glob {
    {
        $(fn $x:ident;)+
        const $all_updaters:ident: [&$updater_type:ident; $count:ident];
    } => {
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
                        tokio::spawn([<$x:lower>]::update())
                    };
                )+

                [$(&[<$x:lower _ updater>]),+]
            }
        };
    };
}

macro_rules! impl_update {
    ($self:ident) => {
        pub async fn update() {
            use hko::{common::Lang, fetch};
            use paste::paste;

            let chinese = match fetch(Lang::TC).await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            let english = match fetch(Lang::EN).await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("{e}");
                    return;
                }
            };

            paste! { let b = [< $self:camel >] ::new(chinese, english); }

            {
                paste! { let arc = super:: [< $self:lower >] (); }
                let mut lock = arc.write().await;
                *lock = b;
            }
        }
    };
}

pub(super) use count_tt;
pub(super) use glob;
pub(super) use impl_update;
