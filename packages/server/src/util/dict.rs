use console::style;
use indicatif::ProgressBar;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::time::Duration;
use zip::ZipArchive;

use anyhow::{Context, bail};

use crate::db::Db;
use crate::schemas::dictionary_index::DictionaryIndex;
use crate::schemas::dictionary_tag_bank_v3::DictionaryTagBankV3;
use crate::schemas::dictionary_term_bank_v3::DictionaryTermBankV3;
use crate::util::config::Config;
use crate::util::progress::get_progress_bar;
pub struct Dict<'a> {
    pub config: &'a Config,
}

impl<'a> Dict<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn parse_dict(&self, dictionary: String, db: &Db<'a>) -> anyhow::Result<()> {
        println!("{} Extracting...", style("[1/3]").bold().dim(),);
        let dict_extract_path = self.extract_dict(dictionary)?;

        println!("{} Parsing...", style("[2/3]").bold().dim());
        let entries =
            fs::read_dir(&dict_extract_path).context("Failed to read dictionary directory")?;
        let entries: Result<Vec<_>, _> = entries.collect();
        let entries = entries?;

        //TODO: optimize memory usage
        let index = self.parse_index(dict_extract_path.join("index.json"))?;
        let all_terms = Self::parse_term_bank(self, &entries)?;
        let all_tags = Self::parse_tag_bank(self, &entries)?;

        println!("{} Inserting...", style("[3/3]").bold().dim());
        db.insert_dictionary_data(&index, &all_terms, &all_tags)
            .await?;

        Ok(())
    }

    fn parse_index(&self, index: PathBuf) -> anyhow::Result<DictionaryIndex> {
        let content = fs::read_to_string(&index);
        let index: DictionaryIndex = serde_json::from_str(&content?)?;
        Ok(index)
    }

    fn parse_term_bank(&self, entries: &[DirEntry]) -> anyhow::Result<DictionaryTermBankV3> {
        let entries = self.get_entries(entries, "term_bank_".to_string())?;
        let mut all_terms = Vec::new();

        let pb = get_progress_bar(entries.len() as u64);
        for entry in entries {
            let content = fs::read_to_string(&entry);
            let file_name = &entry.file_name().unwrap_or(OsStr::new("never"));
            pb.set_message(format!("{}", &file_name.to_string_lossy()));
            match content {
                Ok(content) => {
                    let mut terms: DictionaryTermBankV3 = serde_json::from_str(&content)?;
                    all_terms.append(&mut terms);
                }
                Err(e) => {
                    bail!("Failed to read {}: {}", entry.display(), e)
                }
            }
            pb.inc(1);
        }
        pb.finish_and_clear();

        Ok(all_terms)
    }

    fn parse_tag_bank(&self, entries: &[DirEntry]) -> anyhow::Result<DictionaryTagBankV3> {
        let entries = self.get_entries(entries, "tag_bank_".to_string())?;
        let mut all_tags = Vec::new();
        for entry in entries {
            let content = fs::read_to_string(&entry);
            match content {
                Ok(content) => {
                    let mut terms: DictionaryTagBankV3 = serde_json::from_str(&content)?;
                    all_tags.append(&mut terms);
                }
                Err(e) => {
                    bail!("Failed to read {}: {}", entry.display(), e)
                }
            }
        }
        Ok(all_tags)
    }

    fn get_entries(&self, entries: &[DirEntry], prefix: String) -> anyhow::Result<Vec<PathBuf>> {
        let entries: Vec<PathBuf> = entries
            .iter()
            .filter(|e| {
                let file_name = e.file_name();
                let file_name = file_name.to_str();
                if let Some(file_name) = file_name
                    && file_name.starts_with(&prefix)
                    && file_name.ends_with(".json")
                {
                    return true;
                }
                false
            })
            .map(|e| e.path())
            .collect();
        Ok(entries)
    }

    fn extract_dict(&self, dictionary: String) -> anyhow::Result<PathBuf> {
        let dict_file_name = Path::new(&dictionary)
            .file_name()
            .context("Failed to get file name of dict")?;
        let dict_extract_path = self.config.dir.temp.join(dict_file_name);
        fs::create_dir_all(&dict_extract_path).context("Failed to create temp dir")?;

        let dict_file = fs::File::open(&dictionary)?;
        let mut dict_archive = ZipArchive::new(&dict_file)?;

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message(format!("{}", dict_file_name.to_string_lossy()));
        dict_archive.extract(&dict_extract_path)?;
        pb.finish_and_clear();

        Ok(dict_extract_path)
    }
}
