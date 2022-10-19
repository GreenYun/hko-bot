// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::{mem, sync::Arc};

use tokio::{
    signal,
    sync::{Mutex, RwLock},
    time::{sleep, Duration},
};

use macros::{count_tt, glob};

glob![briefing, bulletin, warning];

pub async fn update() {
    let mutex = Arc::new(Mutex::new(false));
    let thread_mutex = mutex.clone();

    tokio::spawn(async move {
        for updater in all_updaters {
            updater().await.ok();
        }

        let mut it = all_updaters.into_iter().cycle();

        loop {
            match thread_mutex.lock().await {
                dead if *dead => {
                    log::info!("weather updater is shutdown");
                    return;
                }
                _mutex_guard => {
                    if let Some(f) = it.next() {
                        f().await.ok();
                    };
                }
            }

            sleep(Duration::from_secs(60 / (COUNT as u64))).await;
        }
    });

    mem::drop(signal::ctrl_c().await);

    let mut dead = mutex.lock().await;
    *dead = true;

    log::info!("Weather updater shutdown signal sent.");
}

mod briefing;
mod bulletin;
mod warning;

mod macros;
