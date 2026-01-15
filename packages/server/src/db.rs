use sqlx::sqlite::SqlitePool;

use crate::util::config::get_file;

pub async fn init_db() -> anyhow::Result<()> {
    let file = get_file()?.db.to_string_lossy().to_string();
    let file = file + "?mode=rwc";
    let _pool = SqlitePool::connect(&file).await?;
    Ok(())
}
