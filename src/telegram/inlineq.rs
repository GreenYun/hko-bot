// Copyright (c) 2024 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText, ParseMode},
    RequestError,
};

use crate::{
    answer,
    database::types::lang::Lang,
    statics::{BRIEFING_TITLE_CHINESE, BRIEFING_TITLE_ENGLISH, BULLETIN_TITLE_CHINESE, BULLETIN_TITLE_ENGLISH},
};

async fn answer(query: InlineQuery, bot: Bot) -> ResponseResult<()> {
    let mut results = vec![];
    let lang = if query.query.chars().all(|c| c.is_ascii_alphabetic()) {
        Lang::English
    } else {
        Lang::Chinese
    };

    match answer::briefing(&lang).await {
        s if !s.is_empty() => {
            results.push(InlineQueryResult::Article(InlineQueryResultArticle::new(
                "briefing",
                if matches!(lang, Lang::Chinese) {
                    BRIEFING_TITLE_CHINESE
                } else {
                    BRIEFING_TITLE_ENGLISH
                },
                InputMessageContent::Text(InputMessageContentText::new(s).parse_mode(ParseMode::Html)),
            )));
        }
        _ => {}
    };

    match answer::bulletin(&lang).await {
        s if !s.is_empty() => {
            results.push(InlineQueryResult::Article(InlineQueryResultArticle::new(
                "bulletin",
                if matches!(lang, Lang::Chinese) {
                    BULLETIN_TITLE_CHINESE
                } else {
                    BULLETIN_TITLE_ENGLISH
                },
                InputMessageContent::Text(InputMessageContentText::new(s).parse_mode(ParseMode::Html)),
            )));
        }
        _ => {}
    };

    bot.answer_inline_query(query.id, results).await?;

    respond(())
}

pub fn schema() -> UpdateHandler<RequestError> {
    dptree::endpoint(answer)
}
