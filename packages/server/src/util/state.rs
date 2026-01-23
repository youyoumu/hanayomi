use crate::{db::Db, util::lexer::Lexer};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Db>,
    pub lexer: Arc<Lexer>,
}
