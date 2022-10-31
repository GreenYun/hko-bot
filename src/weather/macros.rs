// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

macro_rules! count_tt {
    () => {
        0usize
    };
    ($x:tt $($y:tt)*) => {
        1 + $crate::weather::macros::count_tt!($($y)*)
    };
}

macro_rules! glob {
    [$count:ident; $all_updaters:ident; $($x:ident),+ $(,)?] => {
        $(::paste::paste! {
            ::lazy_static::lazy_static! {
                static ref [<$x:upper>]: Arc<RwLock<[<$x:lower>]::[<$x:camel>]>>
                    = Arc::new(RwLock::new(Default::default()));
            }

            #[inline]
            pub fn [<$x:lower>]() -> Arc<RwLock<[<$x:lower>]::[<$x:camel>]>> {
                [<$x:upper>].clone()
            }

            #[allow(non_upper_case_globals)]
            const [<$x:lower _ updater>]: fn() -> JoinHandle<()> = || {
                tokio::spawn([<$x:lower>]::update())
            };
        })+

        #[allow(non_upper_case_globals)]
        const $count: usize = $crate::weather::macros::count_tt!($($x)+);

        #[allow(non_upper_case_globals)]
        const $all_updaters: [&fn() -> JoinHandle<()>; $count] = [$(::paste::paste!(&[<$x:lower _ updater>])),+];
    };
}

macro_rules! impl_update {
    ($self:ident) => {
        pub async fn update() {
            use hko::{common::Lang, fetch};
            use paste::paste;

            use crate::tool::macros::unwrap_or_execute;

            let chinese = unwrap_or_execute!(fetch(Lang::TC).await, |e| {
                log::error!("{e}");
                return;
            });

            let english = unwrap_or_execute!(fetch(Lang::EN).await, |e| {
                log::error!("{e}");
                return;
            });

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
