// Copyright (c) 2022 - 2026 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::{any::type_name, sync::OnceLock};

use hko::{common::Lang, fetch_with_client};
use tokio::{
	signal::ctrl_c,
	sync::RwLock,
	time::{Duration, sleep},
};

use crate::{http, trigger};

#[allow(clippy::module_name_repetitions)]
pub trait WeatherData: 'static + Clone + std::marker::Sized {
	fn get_store() -> &'static OnceLock<RwLock<Self>>;

	async fn get() -> Option<Self> {
		for _ in 0..3 {
			let data = if let Some(lock) = Self::get_store().get() {
				let lock = lock.read().await;
				Some(lock.clone())
			} else {
				None
			};

			if data.is_some() {
				return data;
			}

			sleep(Duration::from_secs(1)).await;
		}

		None
	}
}

#[allow(clippy::module_name_repetitions)]
trait WeatherDataUpdater: WeatherData + From<(Self::Source, Self::Source)> {
	type Source;

	async fn update(chinese: Self::Source, english: Self::Source) {
		let translated = Self::from((chinese, english));
		if let Some(lock) = Self::get_store().get() {
			let mut lock = lock.write().await;
			*lock = translated;
		} else {
			Self::get_store().set(RwLock::new(translated)).ok();
		}
	}
}

// This allow notation is not good, but we are trying not to use the
// incomplete feature "return_type_notation".
#[allow(clippy::future_not_send)]
async fn update_data<T>()
where
	T: WeatherDataUpdater + Send + Sync,
	T::Source: hko::Fetch + Send,
	// T::update(): Send,
{
	log::debug!("updating {}", type_name::<T>());

	let chinese = fetch_with_client(Lang::TC, http::client()).await;
	let english = fetch_with_client(Lang::EN, http::client()).await;

	if let (Ok(chinese), Ok(english)) = (chinese, english) {
		T::update(chinese, english).await;
		tokio::spawn(trigger::trigger());
	} else {
		log::error!("failed to fetch data for {}", type_name::<T>());
	}
}

pub async fn update() {
	const UPDATE_PERIOD: u64 = 300;

	for updater in ALL_UPDATERS {
		updater().await.ok();
	}

	for updater in ALL_UPDATERS.into_iter().cycle() {
		const SLEEP_TIME: Duration = Duration::from_secs(UPDATE_PERIOD / (COUNT as u64));

		let ctrl_c = ctrl_c();

		tokio::select! {
			r = ctrl_c => {
				if let Err(e) = r {
					log::error!("failed to listen for ^C signal: {e}");
					continue;
				}

				log::info!("^C received, weather updater will be shut down");
				break;
			}

			() = sleep(SLEEP_TIME) => {
				// continue to updater
			}
		}

		updater().await.ok();
	}

	log::info!("weather updater has been shut down");
}

macros::weather_mods! {
	pub mod briefing;
	pub mod bulletin;
	pub mod forecast;
	pub mod warning;
	const ALL_UPDATERS: [&Updater; COUNT];
}

mod macros;
