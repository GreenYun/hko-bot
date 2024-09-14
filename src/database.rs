// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-identifier: MIT

use log::LevelFilter;
use sqlx::{
	postgres::{PgConnectOptions, PgPool},
	ConnectOptions, Error, Row,
};

#[derive(Clone)]
pub struct Connection {
	pool: PgPool,
}

impl Connection {
	pub async fn new<S>(uri: S) -> Result<Self, Error>
	where
		S: Into<String> + Send + Sync,
	{
		let conn_opt = uri
			.into()
			.parse::<PgConnectOptions>()
			.unwrap_or_else(|e| {
				log::error!("{e}");
				panic!("{e}")
			})
			.log_statements(LevelFilter::Debug)
			.log_slow_statements(LevelFilter::Debug, std::time::Duration::default());

		Ok(Self { pool: PgPool::connect_with(conn_opt).await? })
	}
}

pub async fn connect<S>(uri: S) -> Connection
where
	S: Into<String> + Send + Sync,
{
	log::info!("Connecting to database...");

	let db = Connection::new(uri).await.unwrap_or_else(|e| {
		log::error!("{e}");
		panic!("{e}")
	});

	{
		let db = db.clone();

		tokio::spawn(async move {
			let db_name: String = sqlx::query("SELECT current_database()")
				.fetch_one(&db.pool)
				.await
				.map(|row| row.try_get(0).unwrap_or_default())
				.unwrap_or_default();
			if !db_name.is_empty() {
				log::info!("Connected to database {db_name}");
			}
		});
	}

	db
}

pub mod entities;
pub mod types;
