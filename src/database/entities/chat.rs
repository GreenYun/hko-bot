// Copyright (c) 2022 - 2023 GreenYun Organization
// SPDX-License-identifier: MIT

use sqlx::{postgres::PgQueryResult, Error, FromRow};

use crate::database::{types::lang::Lang, Connection};

// CREATE TABLE chat (id bigint PRIMARY KEY, lang lang NOT NULL);
#[derive(Clone, FromRow)]
pub struct Chat {
    pub id: i64,
    pub lang: Lang,
}

impl Connection {
    pub async fn insert_chat(&self, chat: &Chat) -> Result<PgQueryResult, Error> {
        sqlx::query("INSERT INTO chat (id, lang) VALUES ($1, $2)")
            .bind(chat.id)
            .bind(&chat.lang)
            .execute(&self.pool)
            .await
    }

    pub async fn delete_chat(&self, chat_id: i64) -> Result<PgQueryResult, Error> {
        sqlx::query("DELETE FROM chat WHERE id = $1")
            .bind(chat_id)
            .execute(&self.pool)
            .await
    }

    pub async fn select_chat(&self, chat_id: i64) -> Result<Option<Chat>, Error> {
        sqlx::query_as("SELECT id, lang FROM chat WHERE id = $1")
            .bind(chat_id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update_chat(&self, chat: &Chat) -> Result<PgQueryResult, Error> {
        sqlx::query("UPDATE chat SET lang = $1 WHERE id = $2")
            .bind(&chat.lang)
            .bind(chat.id)
            .execute(&self.pool)
            .await
    }
}
