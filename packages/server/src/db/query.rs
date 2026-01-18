use crate::schemas::dictionary_index::DictionaryIndex;
use crate::schemas::dictionary_tag_bank_v3::DictionaryTagBankV3;
use crate::schemas::dictionary_term_bank_v3::DictionaryTermBankV3;
use sqlx::Row;

use super::*;

impl<'a> Db<'a> {
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
                is_updatable, index_url, download_url, tag_meta_json
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

        for entry in entries {
            let defs_json = serde_json::to_string(&entry.5)?;
            sqlx::query(
                r#"-- sql
                INSERT INTO dictionary_entry (
                    dictionary_id, expression, reading, definitions,
                    rules, score, sequence, definition_tags, expression_tags
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(dictionary_id)
            .bind(&entry.0)
            .bind(&entry.1)
            .bind(defs_json)
            .bind(&entry.3)
            .bind(entry.4)
            .bind(entry.6)
            .bind(&entry.2)
            .bind(&entry.7)
            .execute(&mut *tx)
            .await?;
        }

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
}
