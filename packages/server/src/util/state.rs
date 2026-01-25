use crate::{
    db::Db,
    util::{config::Config, lexer::Lexer},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Db>,
    pub lexer: Arc<Lexer>,
    pub config: Arc<Config>,
}
