use crate::{
    db::tables::DefinitionTag,
    util::{
        response::{HandlerResult, RejectionResponse, success},
        state::AppState,
    },
};
use axum::extract::{Query, State};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct IndexQueryParams {
    #[validate(length(min = 1))]
    pub name: String,
}

pub async fn index(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<IndexQueryParams>, RejectionResponse>,
) -> HandlerResult<Vec<DefinitionTag>> {
    params.validate()?;
    let name = params.name;

    let tags = state.db.query_definition_tag_by(name).await?;
    success(tags)
}
