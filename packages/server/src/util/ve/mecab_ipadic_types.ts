/**
 * Japanese Part-of-Speech (POS) types from MeCab IPADIC dictionary
 * These are serialized as Japanese strings (e.g., "名詞", "固有名詞", etc.)
 */
export type POS =
  | "名詞" // Noun
  | "固有名詞" // Proper Noun
  | "代名詞" // Pronoun
  | "助動詞" // Auxiliary Verb
  | "数" // Number
  | "助詞" // Particle
  | "接頭詞" // Prefix Word
  | "動詞" // Verb
  | "記号" // Symbol
  | "フィラー" // Filler
  | "その他" // Other
  | "感動詞" // Interjection
  | "連体詞" // Adnominal
  | "接続詞" // Conjunction
  | "副詞" // Adverb
  | "接続助詞" // Conjunction Particle
  | "形容詞" // Adjective
  | "非自立" // Non-independent
  | "副詞可能" // Adverbial
  | "サ変接続" // Sa-irregular Conjunction
  | "形容動詞語幹" // Na-adjective Stem
  | "ナイ形容詞語幹" // Nai-adjective Stem
  | "助動詞語幹" // Auxiliary Verb Stem
  | "副詞化" // Adverbial Form
  | "体言接続" // Nominal Conjunction
  | "連体化" // Adnominal Form
  | "特殊" // Special
  | "接尾" // Suffix
  | "接続詞的" // Conjunctional
  | "動詞非自立的" // Non-independent Verb
  | "サ変・スル" // Sa-irregular + Suru
  | "特殊・タ" // Special + Ta
  | "特殊・ナイ" // Special + Nai
  | "特殊・タイ" // Special + Tai
  | "特殊・デス" // Special + Desu
  | "特殊・ダ" // Special + Da
  | "特殊・マス" // Special + Masu
  | "特殊・ヌ" // Special + Nu
  | "不変化型" // Uninflected
  | "人名" // Personal Name
  | "命令ｉ" // Imperative I
  | "係助詞" // Binding Particle
  | "*" // Unset/Unknown placeholder
  | "未知"; // Unknown

/**
 * A prepared token with linguistic analysis information
 */
export interface PreparedToken {
  /** The surface form (literal text) of the token */
  literal: string;
  /** Primary part-of-speech classification */
  pos: POS;
  /** Secondary part-of-speech classification */
  pos2: POS;
  /** Tertiary part-of-speech classification */
  pos3: POS;
  /** Quaternary part-of-speech classification (currently unused) */
  pos4: POS;
  /** Inflection type classification */
  inflectionType: POS;
  /** Inflection form classification */
  inflectionForm: POS;
  /** Dictionary form (lemma) of the token */
  lemma: string;
  /** Reading pronunciation (hiragana) */
  reading: string;
  /** Phonetic transcription */
  hatsuon: string;
}

/**
 * Simplified part-of-speech classification for easier consumption
 */
export type PartOfSpeech =
  | "noun" // 名詞
  | "properNoun" // 固有名詞
  | "pronoun" // 代名詞
  | "adjective" // 形容詞
  | "adverb" // 副詞
  | "determiner" // 連体詞
  | "preposition" // (Not used in Japanese, but available for compatibility)
  | "postposition" // 助詞
  | "verb" // 動詞
  | "suffix" // 接尾
  | "prefix" // 接頭詞
  | "conjunction" // 接続詞
  | "interjection" // 感動詞
  | "number" // 数
  | "unknown" // Unknown
  | "symbol" // 記号
  | "other"; // その他

/**
 * Grammatical classification for tokens
 */
export type Grammar =
  | "auxiliary" // Auxiliary function
  | "nominal"; // Nominal function

/**
 * Additional linguistic information for a word
 */
export interface WordExtra {
  /** Reading pronunciation (hiragana) */
  reading: string;
  /** Phonetic transcription */
  transcription: string;
  /** Optional grammatical classification */
  grammar?: Grammar;
}

/**
 * A complete analyzed word with tokens and metadata
 */
export interface Word {
  /** The surface form of the word */
  word: string;
  /** Dictionary form (lemma) - may be null if same as word */
  lemma?: string | null;
  /** Simplified part-of-speech classification */
  partOfSpeech: PartOfSpeech;
  /** All tokens that make up this word */
  tokens: PreparedToken[];
  /** Additional linguistic information */
  extra: WordExtra;
}

/**
 * Vibrato token abstraction (for internal use)
 */
export interface VibratoToken {
  /** Surface form of the token */
  surface: string;
  /** Feature string from MeCab dictionary */
  feature: string;
}

/**
 * Utility functions for working with linguistic data
 */
export const POS = {
  /** Check if a POS value represents a noun-like category */
  isNoun: (pos: POS): boolean => {
    return pos === "名詞" || pos === "固有名詞" || pos === "代名詞";
  },

  /** Check if a POS value represents a verb-like category */
  isVerb: (pos: POS): boolean => {
    return pos === "動詞" || pos === "助動詞";
  },

  /** Check if a POS value represents an adjective-like category */
  isAdjective: (pos: POS): boolean => {
    return pos === "形容詞" || pos === "形容動詞語幹" || pos === "ナイ形容詞語幹";
  },

  /** Check if a POS value represents an adverb-like category */
  isAdverb: (pos: POS): boolean => {
    return pos === "副詞" || pos === "副詞可能" || pos === "副詞化";
  },
} as const;
