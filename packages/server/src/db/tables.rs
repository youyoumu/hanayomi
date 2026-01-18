use crate::schemas::{dictionary_index::TagMeta, dictionary_term_bank_v3::Definition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub revision: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub attribution: Option<String>,
    pub url: Option<String>,

    // Core data settings
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub frequency_mode: Option<String>,

    // Yomitan specific
    pub format: i32, // Resolved from either 'format' or 'version'
    pub sequenced: bool,
    pub minimum_yomitan_version: Option<String>,

    // Update Information
    pub is_updatable: bool,
    pub index_url: Option<String>,
    pub download_url: Option<String>,

    // Metadata / Obsolete fields stored as JSON string if needed
    // Otherwise, you can leave this out if you use separate Tag files
    pub tag_meta_json: Option<TagMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dictionary_id: i32,

    pub expression: String,
    pub reading: String,
    pub definitions: Vec<Definition>,
    pub rules: String,
    pub score: f32,
    pub sequence: i32,
    pub definition_tags: String,
    pub expression_tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionTag {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dictionary_id: i32,

    pub name: String,
    pub category: String,
    pub order: f32,
    pub notes: String,
    pub score: f32,
}
