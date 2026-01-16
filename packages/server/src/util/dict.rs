use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use anyhow::Context;

use crate::schemas::dictionary_term_bank_v3::DictionaryTermBankV3;
use crate::util::config::Config;
pub struct Dict<'a> {
    pub config: &'a Config,
}

impl<'a> Dict<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn parse_dict(&self, dictionary: String) -> anyhow::Result<()> {
        let dict_extract_path = Self::extract_dict(self, dictionary)?;

        let term_bank_1 = fs::read_to_string(dict_extract_path.join("term_bank_1.json"))?;
        let terms: DictionaryTermBankV3 = serde_json::from_str(&term_bank_1)?;
        for term in terms.iter() {
            term.test();
        }

        Ok(())
    }

    pub fn extract_dict(&self, dict: String) -> anyhow::Result<PathBuf> {
        let dict_file_name = Path::new(&dict)
            .file_name()
            .context("Failed to get file name of dict")?;
        let dict_extract_path = self.config.dir.temp.join(dict_file_name);
        fs::create_dir_all(&dict_extract_path).context("Failed to create temp dir")?;

        // let dict_file = fs::File::open(&dict)?;
        // let mut dict_archive = ZipArchive::new(&dict_file)?;
        // dict_archive.extract(&dict_extract_path)?;
        Ok(dict_extract_path)
    }
}
