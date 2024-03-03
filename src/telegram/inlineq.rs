// Copyright (c) 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use std::ops::Not;

use teloxide::{dispatching::UpdateHandler, prelude::*, types::InlineQueryResult, RequestError};

use crate::{
    answer,
    database::types::lang::Lang,
    statics::{BRIEFING_TITLE_CHINESE, BRIEFING_TITLE_ENGLISH, BULLETIN_TITLE_CHINESE, BULLETIN_TITLE_ENGLISH},
};

fn new_result_article<S1, S2, S3>(id: S1, title: S2, content: S3) -> InlineQueryResult
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText, ParseMode};

    InlineQueryResult::Article(InlineQueryResultArticle::new(
        id,
        title,
        InputMessageContent::Text(InputMessageContentText::new(content).parse_mode(ParseMode::Html)),
    ))
}

fn some_non_empty_string(s: String) -> Option<String> {
    s.is_empty().not().then_some(s)
}

async fn answer(query: InlineQuery, bot: Bot) -> ResponseResult<()> {
    let mut results = vec![];

    if let Some(s) = some_non_empty_string(answer::briefing(&Lang::Chinese).await) {
        results.push(new_result_article("briefing", BRIEFING_TITLE_CHINESE, s));
    }

    if let Some(s) = some_non_empty_string(answer::bulletin(&Lang::Chinese).await) {
        results.push(new_result_article("bulletin", BULLETIN_TITLE_CHINESE, s));
    }

    if let Some(s) = some_non_empty_string(answer::briefing(&Lang::English).await) {
        results.push(new_result_article("briefing", BRIEFING_TITLE_ENGLISH, s));
    }

    if let Some(s) = some_non_empty_string(answer::bulletin(&Lang::English).await) {
        results.push(new_result_article("bulletin", BULLETIN_TITLE_ENGLISH, s));
    }

    bot.answer_inline_query(query.id, results).await?;

    respond(())
}

pub fn schema() -> UpdateHandler<RequestError> {
    dptree::endpoint(answer)
}
