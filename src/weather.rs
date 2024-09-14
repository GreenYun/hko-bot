// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{any::type_name, sync::Arc};

use tokio::{
	signal,
	sync::Notify,
	time::{self, Duration},
};

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
	// fn translate(chinese: Self::Source, english: Self::Source) -> Self;
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
	use hko::{common::Lang, fetch_with_client};

	use crate::http;

	let client = http::client();

	log::debug!("updating {}", type_name::<T>());

	let chinese = match fetch_with_client(Lang::TC, client.clone()).await {
		Ok(data) => data,
		Err(e) => {
			log::error!("failed to fetch Chinese weather data: {}", e);
			return;
		}
	};

	let english = match fetch_with_client(Lang::EN, client).await {
		Ok(data) => data,
		Err(e) => {
			log::error!("failed to fetch English weather data: {}", e);
			return;
		}
	};

	T::update(chinese, english).await;
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
					log::info!("Weather updater is shut down");
					return;
				}

				updater().await.ok();
			}

			let mut iter = ALL_UPDATERS.into_iter().cycle();
			loop {
				const SLEEP_TIME: Duration = Duration::from_secs(UPDATE_PERIOD / (COUNT as u64));
				let to = time::timeout(SLEEP_TIME, notified.as_mut()).await;

				if to.is_ok() {
					log::info!("weather updater is shut down");
					return;
				}

				if let Some(f) = iter.next() {
					f().await.ok();
				}
			}
		})
	};

	if let Err(e) = signal::ctrl_c().await {
		log::error!("failed to listen for ctrl-c: {}", e);
	}

	time::sleep(Duration::from_secs(1)).await;
	dead_notify.notify_waiters();
	log::info!("Weather updater shutdown signal sent");
	tokio::join!(handle).0.ok();
}
