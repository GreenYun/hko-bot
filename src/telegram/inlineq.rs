// Copyright (c) 2024 - 2025 GreenYun Organization
// SPDX-License-identifier: MIT

use teloxide::{dispatching::UpdateHandler, prelude::*, types::InlineQueryResult, RequestError};

use crate::{
	answer::{Answer as _, Briefing, Bulletin, Forecast},
	database::types::lang::Lang,
	statics::{
		BRIEFING_TITLE_CHINESE, BRIEFING_TITLE_ENGLISH, BULLETIN_TITLE_CHINESE, BULLETIN_TITLE_ENGLISH,
		FORECAST_TITLE_CHINESE, FORECAST_TITLE_ENGLISH,
	},
	tool::ext::NonEmptyExt as _,
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

	if let Some(s) = Bulletin::answer(&Lang::Chinese).await.get_non_empty() {
		results.push(new_result_article("bulletin_zh", BULLETIN_TITLE_CHINESE, s));
	}

	if let Some(s) = Forecast::answer(&Lang::Chinese).await.join("\n\n").get_non_empty() {
		results.push(new_result_article("forecast_zh", FORECAST_TITLE_CHINESE, s));
	}

	if let Some(s) = Briefing::answer(&Lang::English).await.get_non_empty() {
		results.push(new_result_article("briefing_en", BRIEFING_TITLE_ENGLISH, s));
	}

	if let Some(s) = Bulletin::answer(&Lang::English).await.get_non_empty() {
		results.push(new_result_article("bulletin_en", BULLETIN_TITLE_ENGLISH, s));
	}

	if let Some(s) = Forecast::answer(&Lang::English).await.join("\n\n").get_non_empty() {
		results.push(new_result_article("forecast_en", FORECAST_TITLE_ENGLISH, s));
	}

	bot.answer_inline_query(query.id, results).await?;

	respond(())
}

pub fn schema() -> UpdateHandler<RequestError> {
	dptree::endpoint(answer)
}
