mod query;
mod tables;

use crate::util::config::Config;
use sqlx::sqlite::SqlitePool;

pub struct Db<'a> {
    config: &'a Config,
    pool: SqlitePool,
}

impl<'a> Db<'a> {
    pub async fn new(config: &'a Config) -> anyhow::Result<Self> {
        let file = config.file.db.to_string_lossy().to_string();
        let file = file + "?mode=rwc";
        let pool = SqlitePool::connect(&file).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { config, pool })
    }
}
