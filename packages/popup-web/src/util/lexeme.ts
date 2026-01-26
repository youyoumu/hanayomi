import type { Lexeme } from "@repo/server/types/mecab-ipadic";

export class LexemesProcessor {
  #offsets: number[] = [];
  #lexemes: Lexeme[];

  static cache = new WeakMap<Lexeme[], LexemesProcessor>();
  static new(lexemes: Lexeme[]): LexemesProcessor {
    const cache = LexemesProcessor.cache;
    let instance: LexemesProcessor;
    if (cache.has(lexemes)) {
      instance = cache.get(lexemes)!;
    } else {
      instance = new LexemesProcessor(lexemes);
      cache.set(lexemes, instance);
    }
    return instance;
  }

  private constructor(lexemes: Lexeme[]) {
    this.#lexemes = lexemes;
    let currentLength = 0;
    for (const l of lexemes) {
      this.#offsets.push(currentLength);
      currentLength += l.word.length;
    }
  }

  getLexemeIndex(globalIndex: number): number {
    let low = 0;
    let high = this.#offsets.length - 1;

    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      const start = this.#offsets[mid]!;
      const end = start + this.#lexemes[mid]!.word.length;

      if (globalIndex >= start && globalIndex < end) {
        return mid;
      } else if (globalIndex < start) {
        high = mid - 1;
      } else {
        low = mid + 1;
      }
    }
    return -1;
  }

  getWordClipped(globalIndex: number): string | null {
    const index = this.getLexemeIndex(globalIndex);
    if (index === -1) return null;

    const lexeme = this.#lexemes[index]!;
    const startOffset = this.#offsets[index]!;
    const relativeOffset = globalIndex - startOffset;
    return lexeme.word.slice(relativeOffset);
  }

  getFirstTokenLemma(word: Lexeme) {
    return word.tokens[0]?.lemma;
  }
}
