// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use teloxide::{prelude::*, types::ParseMode};

use super::macros::reply_html;
use crate::{
    answer::{Answer, Briefing},
    database::entities::chat::Chat,
};

pub(super) async fn briefing(message: Message, bot: Bot, chat: Chat) -> ResponseResult<()> {
    let chat_id = message.chat.id;

    reply_html!(chat_id, message.id, Briefing::answer(&chat.lang).await, bot)?;

    respond(())
}
