use std::fs::File;
use vibrato::Dictionary;
use vibrato::Tokenizer;
pub struct Lexer {
    tokenizer: Tokenizer,
}

impl Lexer {
    pub fn new() -> anyhow::Result<Self> {
        let reader = File::open("./.vibrato_models/ipadic-mecab-2_7_0/system.dic.zst")?;
        let reader = zstd::Decoder::new(reader)?;
        let dict = Dictionary::read(reader)?;
        let tokenizer = Tokenizer::new(dict);
        let lexer = Self { tokenizer };
        Ok(lexer)
    }

    pub fn tokenize(&self, text: String) -> Vec<String> {
        let mut worker = self.tokenizer.new_worker();
        worker.reset_sentence(text);
        worker.tokenize();
        let tokens: Vec<_> = worker
            .token_iter()
            .map(|t| t.feature().to_string())
            .collect();
        tokens
    }
}
