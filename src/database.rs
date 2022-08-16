// Copyright (c) 2022 GreenYun Organizaiton
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
        S: ToString,
    {
        let mut conn_opt: PgConnectOptions = uri.to_string().parse().unwrap_or_else(|e| {
            log::error!("{}", e);
            panic!("{}", e)
        });

        conn_opt
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Debug, std::time::Duration::default());

        Ok(Self {
            pool: PgPool::connect_with(conn_opt).await?,
        })
    }
}

pub async fn connect<S>(uri: S) -> Connection
where
    S: ToString,
{
    log::info!("Connecting to database...");

    let db = Connection::new(uri.to_string()).await.unwrap_or_else(|e| {
        log::error!("{}", e);
        panic!("{}", e)
    });

    let db_name: String = sqlx::query("SELECT current_database()")
        .fetch_one(&db.pool)
        .await
        .unwrap()
        .try_get(0)
        .unwrap_or_default();
    log::info!("Connected to database {}", db_name);

    db
}

pub mod entities;
pub mod types;
