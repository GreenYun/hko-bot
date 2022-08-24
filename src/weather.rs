// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::{sync::Arc, time::Duration};

use lazy_static::lazy_static;
use paste::paste;
use tokio::{
    signal,
    sync::{Mutex, RwLock},
    time::sleep,
};

macro_rules! count_tt {
    () => {
        0usize
    };
    ($x:tt $($y:tt)*) => {
        1 + count_tt!($($y)*)
    };
}

macro_rules! glob {
    [$($x:ident),+ $(,)?] => {
        $(paste! {
            lazy_static! {
                static ref [< $x:upper >]: Arc<RwLock< [< $x:lower >] :: [< $x:camel >] >> = Arc::new(RwLock::new(Default::default()));
            }

            #[inline]
            pub fn [< $x:lower >] () -> Arc<RwLock< [< $x:lower >] :: [< $x:camel >] >> {
                [< $x:upper >]
                    .clone()
            }
        })+

        const COUNT: usize = 3 + count_tt!($($x)+);
    };
}

glob![briefing, bulletin];

pub async fn update() {
    let mutex = Arc::new(Mutex::new(true));
    let thread_mutex = mutex.clone();

    tokio::spawn(async move {
        let mut sleep_sec = 0;
        let mut it = (0..COUNT).into_iter().cycle();

        macro_rules! job_select {
            ($i:expr, $x:ident $(, $others:ident)* $(,)?) => {
                match $i {
                    x if x == (count_tt!($($others)*)) => {
                        $x::update().await;
                    }
                    _ => {
                        job_select!($i, $($others,)*);
                    }
                }
            };
            ($i:expr $(,)?) => {
                sleep_sec = 60 / (COUNT as u64);
            }
        }

        loop {
            {
                let mutex = thread_mutex.lock().await;

                if !*mutex {
                    log::info!("weather updater is shutdown");
                    break;
                }

                let i = it.next().unwrap();
                job_select!(i, briefing, bulletin);
            }

            sleep(Duration::from_secs(sleep_sec)).await;
        }
    });

    let _ = signal::ctrl_c().await;

    let mut m = mutex.lock().await;
    *m = false;

    log::info!("Weather updater shutdown signal sent.");
}

mod briefing;
mod bulletin;
