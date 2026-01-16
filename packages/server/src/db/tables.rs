use crate::schemas::{dictionary_index::TagMeta, dictionary_term_bank_v3::Definition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
    pub id: i32,

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
struct DictionaryEntry {
    id: i32,
    dictionary_id: i32,

    expression: String,
    reading: String,
    definitions: Vec<Definition>,
    rules: String,
    score: f32,
    sequence: i32,
    definition_tags: String,
    expression_tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DefinitionTag {
    id: i32,
    dictionary_id: i32,

    name: String,
    category: String,
    order: f32,
    notes: String,
    score: f32,
}
