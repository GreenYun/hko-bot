// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use teloxide::{
    prelude::{respond, AutoSend, Bot, ChatId, Requester},
    requests::ResponseResult,
};

pub async fn start_first(bot: AutoSend<Bot>, chat_id: ChatId) -> ResponseResult<()> {
    bot.send_message(chat_id, "/start first.").await?;

    respond(())
}
