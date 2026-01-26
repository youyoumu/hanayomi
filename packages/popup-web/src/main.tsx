/* @refresh reload */
import { debounce, uniq } from "es-toolkit";
import type { Word } from "@repo/server/types/mecab-ipadic";
import { queries } from "./util/queryKeyFactory";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { render } from "solid-js/web";
import { Popup } from "./components/Popup";
import "./styles/main.css";
import { setupTailwind } from "./util/dev";

class WordLexer {
  #offsets: number[] = [];
  #words: Word[];

  static cache = new WeakMap<Word[], WordLexer>();
  static new(words: Word[]): WordLexer {
    const cache = WordLexer.cache;
    let wordIndexer: WordLexer;
    if (cache.has(words)) {
      wordIndexer = cache.get(words)!;
    } else {
      wordIndexer = new WordLexer(words);
      cache.set(words, wordIndexer);
    }
    return wordIndexer;
  }

  constructor(words: Word[]) {
    this.#words = words;
    let currentLength = 0;
    for (const w of words) {
      this.#offsets.push(currentLength);
      currentLength += w.word.length;
    }
  }

  getWordIndex(globalIndex: number): number {
    let low = 0;
    let high = this.#offsets.length - 1;

    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      const start = this.#offsets[mid]!;
      const end = start + this.#words[mid]!.word.length;

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

  getFirstTokenLemma(word: Word) {
    return word.tokens[0]?.lemma;
  }
}

export function init() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: Infinity,
      },
    },
  });
  const pupup = document.createElement("div");
  const root = document.createElement("div");
  const shadow = pupup.attachShadow({ mode: "closed" });
  setupTailwind(shadow);
  shadow.appendChild(root);
  document.body.appendChild(pupup);

  const scanText = async (e: MouseEvent) => {
    // console.log(e.clientX, e.clientY);
    const result = document.caretPositionFromPoint(e.clientX, e.clientY);
    if (result && result.offsetNode.nodeType === Node.TEXT_NODE) {
      const node = result.offsetNode as Text;
      const offset = result.offset;
      const text = node.data;

      const words = await queryClient.fetchQuery({
        ...queries.tokenize.detail(text),
      });

      const wordLexer = WordLexer.new(words);
      const wordIndex = wordLexer.getWordIndex(offset);
      const word = words[wordIndex];
      if (!word) return;
      const firstTokenLemma = wordLexer.getFirstTokenLemma(word);
      const expressions = uniq([word.word, firstTokenLemma].filter(Boolean)) as string[];

      root.innerHTML = "";
      render(
        () => (
          <QueryClientProvider client={queryClient}>
            <Popup expressions={expressions} />
          </QueryClientProvider>
        ),
        root,
      );
    }
  };
  const dScanText = debounce(scanText, 100);

  document.addEventListener("mousemove", dScanText);
}
