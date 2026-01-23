use std::fs::File;
use vibrato::Dictionary;
use vibrato::Tokenizer;

use crate::util::ve::mecab_ipadic::VibratoToken;
use crate::util::ve::mecab_ipadic::Word;
use crate::util::ve::mecab_ipadic::parse_into_words;
use crate::util::ve::mecab_ipadic::prepare_tokens;

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

    pub fn tokenize(&self, text: String) -> anyhow::Result<Vec<Word>> {
        let mut worker = self.tokenizer.new_worker();
        worker.reset_sentence(text);
        worker.tokenize();
        let tokens: Vec<VibratoToken> = worker.token_iter().map(|t| t.into()).collect();
        let prepared_tokens = prepare_tokens(tokens)?;
        let words = parse_into_words(prepared_tokens)?;
        Ok(words)
    }
}
