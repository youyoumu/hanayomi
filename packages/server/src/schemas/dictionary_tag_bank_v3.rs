use serde::{Deserialize, Serialize};
use serde_valid::Validate;

/// Data file containing tag information for terms and kanji.
pub type DictionaryTagBankV3 = Vec<DictionaryTagBankV3Row>;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct DictionaryTagBankV3Row(
    /// Tag name.
    pub String,
    /// Category for the tag.
    pub String,
    /// Sorting order for the tag.
    pub f32,
    /// Notes for the tag.
    pub String,
    /// Score used to determine popularity. Negative values are more rare and positive values are more frequent.
    pub f32,
);
