// Copyright (c) 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{dispatching::UpdateHandler, prelude::*, types::InlineQueryResult, RequestError};

use crate::{
    answer::{self, Answer, Briefing},
    database::types::lang::Lang,
    statics::{BRIEFING_TITLE_CHINESE, BRIEFING_TITLE_ENGLISH, BULLETIN_TITLE_CHINESE, BULLETIN_TITLE_ENGLISH},
    tool::ext::NonEmptyExt,
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

async fn answer(query: InlineQuery, bot: Bot) -> ResponseResult<()> {
    let mut results = vec![];

    if let Some(s) = Briefing::answer(&Lang::Chinese).await.get_non_empty() {
        results.push(new_result_article("briefing_zh", BRIEFING_TITLE_CHINESE, s));
    }

    if let Some(s) = answer::bulletin(&Lang::Chinese).await.get_non_empty() {
        results.push(new_result_article("bulletin_zh", BULLETIN_TITLE_CHINESE, s));
    }

    if let Some(s) = Briefing::answer(&Lang::English).await.get_non_empty() {
        results.push(new_result_article("briefing_en", BRIEFING_TITLE_ENGLISH, s));
    }

    if let Some(s) = answer::bulletin(&Lang::English).await.get_non_empty() {
        results.push(new_result_article("bulletin_en", BULLETIN_TITLE_ENGLISH, s));
    }

    bot.answer_inline_query(query.id, results).await?;

    respond(())
}

pub fn schema() -> UpdateHandler<RequestError> {
    dptree::endpoint(answer)
}
