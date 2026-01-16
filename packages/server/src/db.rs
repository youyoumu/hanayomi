mod tables;

use crate::util::config::Config;
use sqlx::sqlite::SqlitePool;

pub struct Db<'a> {
    config: &'a Config,
}

impl<'a> Db<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn init_db(&self) -> anyhow::Result<()> {
        let file = self.config.file.db.to_string_lossy().to_string();
        let file = file + "?mode=rwc";
        let _pool = SqlitePool::connect(&file).await?;
        Ok(())
    }
}
