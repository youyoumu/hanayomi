use crate::db::tables::{DefinitionTag, Dictionary, DictionaryEntry};
use crate::schemas::dictionary_index::DictionaryIndex;
use crate::schemas::dictionary_tag_bank_v3::DictionaryTagBankV3;
use crate::schemas::dictionary_term_bank_v3::DictionaryTermBankV3;
use crate::util::progress::get_progress_bar;
use sqlx::Row;

use super::*;

impl Db {
    pub async fn insert_dictionary_data(
        &self,
        dict: &DictionaryIndex,
        entries: &DictionaryTermBankV3,
        tags: &DictionaryTagBankV3,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query(
            r#"--sql
            INSERT INTO dictionary (
                title, revision, author, description, attribution, url,
                source_language, target_language, frequency_mode,
                format, sequenced, minimum_yomitan_version,
                is_updatable, index_url, download_url, tag_meta
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
        )
        .bind(&dict.title)
        .bind(&dict.revision)
        .bind(&dict.author)
        .bind(&dict.description)
        .bind(&dict.attribution)
        .bind(&dict.url)
        .bind(&dict.source_language)
        .bind(&dict.target_language)
        .bind(&dict.frequency_mode)
        .bind(dict.get_format())
        .bind(dict.sequenced)
        .bind(&dict.minimum_yomitan_version)
        .bind(dict.is_updatable)
        .bind(&dict.index_url)
        .bind(&dict.download_url)
        .bind(serde_json::to_string(&dict.tag_meta)?)
        .fetch_one(&mut *tx)
        .await?;
        let dictionary_id: i32 = row.get(0);

        let pb = get_progress_bar(entries.len() as u64);
        let chunk_size = 100;
        for chunk in entries.chunks(chunk_size) {
            let mut query_builder = sqlx::QueryBuilder::new(
                r#"-- sql
                INSERT INTO dictionary_entry (
                    dictionary_id, expression, reading, definitions,
                    rules, score, sequence, definition_tags, expression_tags
                )"#,
            );

            query_builder.push_values(chunk, |mut b, entry| {
                let expression = &entry.0;
                pb.set_message(expression.to_string());
                let defs_json = serde_json::to_string(&entry.5).unwrap();
                b.push_bind(dictionary_id)
                    .push_bind(&entry.0)
                    .push_bind(&entry.1)
                    .push_bind(defs_json)
                    .push_bind(&entry.3)
                    .push_bind(entry.4)
                    .push_bind(entry.6)
                    .push_bind(&entry.2)
                    .push_bind(&entry.7);
            });

            let query = query_builder.build();
            query.execute(&mut *tx).await?;
            pb.inc(chunk_size as u64);
        }
        pb.finish_and_clear();

        for tag in tags {
            sqlx::query(
                r#"-- sql
                INSERT INTO definition_tag (dictionary_id, name, category, "order", notes, score)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(dictionary_id)
            .bind(&tag.0)
            .bind(&tag.1)
            .bind(tag.2)
            .bind(&tag.3)
            .bind(tag.4)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn _vacuum(&self) -> anyhow::Result<()> {
        sqlx::query("VACUUM").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn query_dictionary_entry_by(
        &self,
        expression: String,
    ) -> anyhow::Result<Vec<DictionaryEntry>> {
        let row: Vec<DictionaryEntry> = sqlx::query_as(
            r#"--sql
            SELECT * FROM dictionary_entry WHERE expression = ?
            "#,
        )
        .bind(&expression)
        .fetch_all(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn query_dictionary(&self, dictionary_id: i32) -> anyhow::Result<Option<Dictionary>> {
        let row: Option<Dictionary> = sqlx::query_as(
            r#"--sql
            SELECT * FROM dictionary WHERE id = ?
            "#,
        )
        .bind(dictionary_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn query_definition_tag_by(
        &self,
        name: String,
    ) -> anyhow::Result<Vec<DefinitionTag>> {
        let row: Vec<DefinitionTag> = sqlx::query_as(
            r#"--sql
            SELECT * FROM definition_tag WHERE name = ?
            "#,
        )
        .bind(&name)
        .fetch_all(&self.pool)
        .await?;
        Ok(row)
    }
}
