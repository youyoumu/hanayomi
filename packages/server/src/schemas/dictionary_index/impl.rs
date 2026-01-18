use super::*;

impl DictionaryIndex {
    pub fn get_format(&self) -> u8 {
        if let Some(version) = self.version {
            version
        } else {
            self.format.unwrap_or(3)
        }
    }
}
