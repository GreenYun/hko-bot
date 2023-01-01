// Copyright (c) 2022 GreenYun Organization
// SPDX-License-identifier: MIT

use std::sync::Arc;

use tokio::{
    signal,
    sync::Mutex,
    time::{sleep, Duration},
};

macros::glob! {
    fn briefing;
    fn bulletin;
    fn warning;
    const ALL_UPDATERS: [&Updater; COUNT];
}

pub async fn update() {
    const UPDATE_PERIOD: u64 = 300;

    let mutex = Arc::new(Mutex::new(false));
    let thread_mutex = mutex.clone();

    tokio::spawn(async move {
        for updater in ALL_UPDATERS {
            updater().await.ok();
        }

        let mut it = ALL_UPDATERS.into_iter().cycle();

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

            sleep(Duration::from_secs(UPDATE_PERIOD / (COUNT as u64))).await;
        }
    });

    if let Err(e) = signal::ctrl_c().await {
        log::error!("failed to listen for ctrl-c: {}", e);
    }

    let mut dead = mutex.lock().await;
    *dead = true;

    log::info!("Weather updater shutdown signal sent.");
}

mod briefing;
mod bulletin;
mod warning;

mod macros;
