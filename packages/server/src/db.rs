mod query;
mod tables;

use std::sync::Arc;

use crate::util::config::Config;
use sqlx::sqlite::SqlitePool;

pub struct Db {
    config: Arc<Config>,
    pool: SqlitePool,
}

impl Db {
    pub async fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        let file = config.file.db.to_string_lossy().to_string();
        let file = file + "?mode=rwc";
        let pool = SqlitePool::connect(&file).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { config, pool })
    }
}
