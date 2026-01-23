use crate::util::{
    response::{HandlerResult, RejectionResponse, success},
    state::AppState,
    ve::mecab_ipadic::Word,
};
use axum::extract::{Query, State};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct HandleQueryParams {
    #[validate(length(min = 1))]
    pub sentence: String,
}

pub async fn handle(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<HandleQueryParams>, RejectionResponse>,
) -> HandlerResult<Vec<Word>> {
    params.validate()?;
    let sentence = params.sentence;

    let tokens = state.lexer.tokenize(sentence)?;
    success(tokens)
}
