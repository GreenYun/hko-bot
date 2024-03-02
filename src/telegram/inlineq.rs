// Copyright (c) 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{dispatching::UpdateHandler, prelude::*, RequestError};

use crate::database::Connection;

async fn answer(_query: InlineQuery, _bot: Bot, _db_conn: Connection) -> ResponseResult<()> {
    respond(())
}

pub fn schema() -> UpdateHandler<RequestError> {
    dptree::endpoint(answer)
}
