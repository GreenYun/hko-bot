// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{any::type_name, sync::Arc};

use hko::{common::Lang, fetch_with_client};
use tokio::{
	signal,
	sync::Notify,
	time::{self, Duration},
};

use crate::http;

mod macros;

macros::weather_mods! {
	pub mod briefing;
	pub mod bulletin;
	pub mod warning;
	const ALL_UPDATERS: [&Updater; COUNT];
}

#[allow(clippy::module_name_repetitions)]
pub trait WeatherData: std::marker::Sized {
	async fn get() -> Option<Self>;
}

#[allow(clippy::module_name_repetitions)]
trait WeatherDataUpdater {
	type Source;

	async fn update(chinese: Self::Source, english: Self::Source);
}

// This allow notation is not good, but we are trying to not to use the
// incomplete feature "return_type_notation".
#[allow(clippy::future_not_send)]
async fn update_data<T>()
where
	T: WeatherDataUpdater + Send + Sync,
	T::Source: hko::Fetch + Send,
	// T::update(): Send,
{
	let client = http::client();

	log::debug!("updating {}", type_name::<T>());

	let chinese = fetch_with_client(Lang::TC, client.clone()).await;

	let english = fetch_with_client(Lang::EN, client).await;

	if let (Ok(chinese), Ok(english)) = (chinese, english) {
		T::update(chinese, english).await;
	} else {
		log::error!("failed to fetch data for {}", type_name::<T>());
	}
}

pub async fn update() {
	const UPDATE_PERIOD: u64 = 300;

	let dead_notify = Arc::new(Notify::new());

	let handle = {
		let dead_notify = dead_notify.clone();
		tokio::spawn(async move {
			let notified = dead_notify.notified();
			tokio::pin!(notified);

			for updater in ALL_UPDATERS {
				if notified.as_mut().enable() {
					log::info!("weather updater is shut down");
					return;
				}

				updater().await.ok();
			}

			for updater in ALL_UPDATERS.into_iter().cycle() {
				const SLEEP_TIME: Duration = Duration::from_secs(UPDATE_PERIOD / (COUNT as u64));
				let to = time::timeout(SLEEP_TIME, notified.as_mut()).await;

				if to.is_ok() {
					log::info!("weather updater is shut down");
					return;
				}

				updater().await.ok();
			}
		})
	};

	if let Err(e) = signal::ctrl_c().await {
		log::error!("failed to listen for ctrl-c: {}", e);
	}

	time::sleep(Duration::from_secs(1)).await;
	dead_notify.notify_waiters();
	log::info!("weather updater shutdown signal sent");
	tokio::join!(handle).0.ok();
}
