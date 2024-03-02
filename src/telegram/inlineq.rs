// Copyright (c) 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{dispatching::UpdateHandler, prelude::*, RequestError};

use crate::database::Connection;

async fn answer(query: InlineQuery, bot: Bot, db_conn: Connection) -> ResponseResult<()> {
    todo!()
}

pub fn schema() -> UpdateHandler<RequestError> {
    dptree::endpoint(answer)
}
