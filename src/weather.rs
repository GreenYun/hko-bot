// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::Arc;

use tokio::{
    signal,
    sync::Mutex,
    time::{sleep, Duration},
};

macros::glob! {
    mod briefing;
    mod bulletin;
    mod warning;
    const ALL_UPDATERS: [&Updater; COUNT];
}

pub async fn update() {
    const UPDATE_PERIOD: u64 = 300;

    let mutex = Arc::new(Mutex::new(false));
    let inner_mutex = mutex.clone();

    tokio::spawn(async move {
        {
            let _ = inner_mutex.lock().await;
            for updater in ALL_UPDATERS {
                updater().await.ok();
            }
        }

        let mut iter = ALL_UPDATERS.into_iter().cycle();
        loop {
            {
                let will_die = inner_mutex.lock().await;
                if *will_die {
                    log::info!("weather updater is shut down");
                    return;
                } else if let Some(f) = iter.next() {
                    f().await.ok();
                }
            }

            sleep(Duration::from_secs(UPDATE_PERIOD / (COUNT as u64))).await;
        }
    });

    if let Err(e) = signal::ctrl_c().await {
        log::error!("failed to listen for ctrl-c: {}", e);
    }

    {
        let mut will_die = mutex.lock().await;
        *will_die = true;
    }

    log::info!("Weather updater shutdown signal sent");
}

mod macros;
