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

type Updater = fn() -> tokio::task::JoinHandle<()>;

const ALL_UPDATERS: &[Updater] = &[
	launch_update::<briefing::Briefing>,
	launch_update::<bulletin::Bulletin>,
	launch_update::<forecast::Forecast>,
	launch_update::<warning::Warning>,
];

#[allow(clippy::module_name_repetitions)]
pub trait WeatherData: 'static + Clone + From<(Self::Source, Self::Source)> {
	type Source;

	fn get_store() -> &'static OnceLock<RwLock<Self>>;

	async fn get() -> Option<Self> {
		const MAX_ATTEMPTS: usize = 3;
		const RETRY_DELAY: Duration = Duration::from_secs(1);

		for attempt in 0..MAX_ATTEMPTS {
			if let Some(lock) = Self::get_store().get() {
				return Some(lock.read().await.clone());
			}

			if attempt + 1 < MAX_ATTEMPTS {
				sleep(RETRY_DELAY).await;
			}
		}

		None
	}
}

async fn update_data<T>()
where
	T: WeatherData + Send + Sync,
	T::Source: hko::Fetch + Send,
{
	log::debug!("updating {}", type_name::<T>());

	let client = http::client();
	let (chinese, english) =
		tokio::join!(fetch_with_client(Lang::TC, client.clone()), fetch_with_client(Lang::EN, client),);

	if let (Ok(chinese), Ok(english)) = (chinese, english) {
		let translated = T::from((chinese, english));
		if let Some(lock) = T::get_store().get() {
			let mut lock = lock.write().await;
			*lock = translated;
		} else {
			T::get_store().set(RwLock::new(translated)).ok();
		}
		tokio::spawn(trigger::trigger());
	} else {
		log::error!("failed to fetch data for {}", type_name::<T>());
	}
}

fn launch_update<T>() -> tokio::task::JoinHandle<()>
where
	T: WeatherData + Send + Sync,
	T::Source: hko::Fetch + Send,
{
	tokio::spawn(update_data::<T>())
}

pub async fn update() {
	const UPDATE_PERIOD: u64 = 300;

	let ctrl_c = ctrl_c();
	tokio::pin!(ctrl_c);

	for updater in ALL_UPDATERS {
		updater().await.ok();
	}

	for updater in ALL_UPDATERS.iter().cycle() {
		const SLEEP_TIME: Duration = Duration::from_secs(UPDATE_PERIOD / ALL_UPDATERS.len() as u64);

		tokio::select! {
			r = &mut ctrl_c => {
				match r {
					Ok(()) => log::info!("^C received, weather updater will be shut down"),
					Err(e) => log::error!("failed to listen for ^C signal: {e}"),
				}
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

pub mod briefing;
pub mod bulletin;
pub mod forecast;
pub mod warning;
