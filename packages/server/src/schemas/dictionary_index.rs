use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

// TODO: serde_valie .validate needs to be invoked manually
// TODO: anyOf and dependencies validation needs to be implemented

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryIndex {
    /// Title of the dictionary.
    pub title: String,
    /// Revision of the dictionary.
    pub revision: String,
    /// Minimum version of Yomitan that is compatible with this dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_yomitan_version: Option<String>,
    /// Whether or not this dictionary contains sequencing information.
    #[serde(default)]
    pub sequenced: bool,
    /// Format of data found in the JSON data files.
    #[validate(enumerate = [1, 2, 3])]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<u8>,
    /// Alias for format.
    #[validate(enumerate = [1, 2, 3])]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u8>,
    /// Creator of the dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Whether this dictionary contains links to its latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_updatable: Option<bool>,
    /// URL for the index file of the latest revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_url: Option<String>,
    /// URL for the download of the latest revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// URL for the source of the dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Description of the dictionary data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Attribution information for the dictionary data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// Language of the terms in the dictionary (ISO 639 code).
    #[validate(pattern = r"^[a-z]{2,3}$")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// Main language of the definitions (ISO 639 code).
    #[validate(pattern = r"^[a-z]{2,3}$")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    /// Frequency mode for the dictionary.
    #[validate(enumerate = ["occurrence-based", "rank-based"])]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_mode: Option<String>,
    /// Tag information (Obsolete, but included for compatibility).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_meta: Option<TagMeta>,
}

pub type TagMeta = HashMap<String, TagMetaValue>;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct TagMetaValue {
    /// Category for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Sorting order for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<f32>,
    /// Notes for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Score used to determine popularity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

mod r#impl;
