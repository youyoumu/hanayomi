/* @refresh reload */
import { debounce } from "es-toolkit";
import type { Word } from "@repo/server/types/mecab-ipadic";
import { queries } from "./util/queryKeyFactory";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { render } from "solid-js/web";
import { Popup } from "./components/Popup";
import "./styles/main.css";
import { setupTailwind } from "./util/dev";

class WordIndexer {
  private offsets: number[] = [];
  private words: Word[];

  static cache = new WeakMap<Word[], WordIndexer>();
  static new(words: Word[]): WordIndexer {
    const cache = WordIndexer.cache;
    let wordIndexer: WordIndexer;
    if (cache.has(words)) {
      wordIndexer = cache.get(words)!;
    } else {
      wordIndexer = new WordIndexer(words);
      cache.set(words, wordIndexer);
    }
    return wordIndexer;
  }

  constructor(words: Word[]) {
    this.words = words;
    let currentLength = 0;
    for (const w of words) {
      this.offsets.push(currentLength);
      currentLength += w.word.length;
    }
  }

  public getWordIndex(globalIndex: number): number {
    let low = 0;
    let high = this.offsets.length - 1;

    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      const start = this.offsets[mid]!;
      const end = start + this.words[mid]!.word.length;

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

      const wordIndexer = WordIndexer.new(words);
      const wordIndex = wordIndexer.getWordIndex(offset);
      const word = words[wordIndex];
      if (!word) return;
      console.log("DEBUG[1455]: word=", word);

      root.innerHTML = "";
      render(
        () => (
          <QueryClientProvider client={queryClient}>
            <Popup expression={word.word} />
          </QueryClientProvider>
        ),
        root,
      );
    }
  };
  const dScanText = debounce(scanText, 100);

  document.addEventListener("mousemove", dScanText);
}
