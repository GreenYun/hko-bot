// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{any::type_name, sync::Arc};

use tokio::{
    signal,
    sync::Mutex,
    time::{sleep, Duration},
};

macros::weather_mods! {
    mod briefing;
    mod bulletin;
    mod warning;
    const ALL_UPDATERS: [&Updater; COUNT];
}

trait WeatherData {
    type Source;
    const UPDATE_FN: fn() -> std::sync::Arc<tokio::sync::RwLock<Self>>;
    fn translate(chinese: Self::Source, english: Self::Source) -> Self;
}

trait AsyncUpdater {
    async fn update();
}

impl<T> AsyncUpdater for T
where
    T: WeatherData + Send + Sync,
    T::Source: hko::Fetch + Send,
{
    async fn update() {
        use hko::{common::Lang, fetch};

        log::debug!("updating {}", type_name::<T>());

        let chinese = match fetch(Lang::TC).await {
            Ok(data) => data,
            Err(e) => {
                log::error!("failed to fetch Chinese weather data: {}", e);
                return;
            }
        };

        let english = match fetch(Lang::EN).await {
            Ok(data) => data,
            Err(e) => {
                log::error!("failed to fetch English weather data: {}", e);
                return;
            }
        };

        let translated = T::translate(chinese, english);

        {
            let arc = Self::UPDATE_FN();
            let mut lock = arc.write().await;
            *lock = translated;
        }
    }
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
