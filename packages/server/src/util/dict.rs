use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use anyhow::{Context, bail};

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

        // 1. Read the directory entries
        let entries =
            fs::read_dir(&dict_extract_path).context("Failed to read dictionary directory")?;
        let entries: Result<Vec<_>, _> = entries.collect();
        let entries = entries?;
        let all_terms = Self::parse_term_bank(self, entries)?;

        Ok(())
    }

    pub fn parse_term_bank(&self, entries: Vec<DirEntry>) -> anyhow::Result<DictionaryTermBankV3> {
        let entries: Vec<PathBuf> = entries
            .iter()
            .filter(|e| {
                let file_name = e.file_name();
                let file_name = file_name.to_str();
                if let Some(file_name) = file_name
                    && file_name.starts_with("term_bank_")
                    && file_name.ends_with(".json")
                {
                    return true;
                }
                false
            })
            .map(|e| e.path())
            .collect();

        let mut all_terms = Vec::new();
        for entry in entries {
            let content = fs::read_to_string(&entry);
            match content {
                Ok(content) => {
                    let mut terms: DictionaryTermBankV3 = serde_json::from_str(&content)?;
                    all_terms.append(&mut terms);
                }
                Err(e) => {
                    bail!("Failed to read {}: {}", entry.display(), e)
                }
            }
        }

        Ok(all_terms)
    }

    pub fn extract_dict(&self, dictionary: String) -> anyhow::Result<PathBuf> {
        let dict_file_name = Path::new(&dictionary)
            .file_name()
            .context("Failed to get file name of dict")?;
        let dict_extract_path = self.config.dir.temp.join(dict_file_name);
        fs::create_dir_all(&dict_extract_path).context("Failed to create temp dir")?;

        let dict_file = fs::File::open(&dictionary)?;
        let mut dict_archive = ZipArchive::new(&dict_file)?;
        dict_archive.extract(&dict_extract_path)?;
        Ok(dict_extract_path)
    }
}
