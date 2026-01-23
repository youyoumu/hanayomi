import { debounce } from "es-toolkit";
import type { Word } from "@repo/server/types/mecab-ipadic";
import ky from "ky";
import { queries } from "./util/queryKeyFactory";
import { QueryClient } from "@tanstack/solid-query";

function getWordIndexAtGlobalIndex(words: Word[], globalIndex: number): number {
  let currentLength = 0;
  for (let i = 0; i < words.length; i++) {
    const wordLength = words[i].word.length;
    // Check if the globalIndex falls within the bounds of the current word
    if (globalIndex >= currentLength && globalIndex < currentLength + wordLength) {
      return i;
    }
    currentLength += wordLength;
  }
  return -1; // Return -1 if the index is out of bounds
}

/* @refresh reload */
export function init() {
  const tokenizeCache = new Map<string, Word[]>();

  const api = ky.create({
    prefixUrl: "http://localhost:45636",
  });
  const queryClient = new QueryClient();
  const scanText = async (e: MouseEvent) => {
    // console.log(e.clientX, e.clientY);
    const result = document.caretPositionFromPoint(e.clientX, e.clientY);
    if (result && result.offsetNode.nodeType === Node.TEXT_NODE) {
      const node = result.offsetNode as Text;
      const offset = result.offset;
      const text = node.data;
      // console.log(offset, text);

      const response = await api
        .get<{
          result: "success";
          data: Word[];
        }>(`tokenize`, {
          searchParams: {
            sentence: text,
          },
        })
        .json();
      // console.log(response);

      const wordIndex = getWordIndexAtGlobalIndex(response.data, offset);
      const word = response.data[wordIndex];
      console.log("DEBUG[1422]: word=", word.word);
      const response2 = await api
        .get("dictionary_entries/search", {
          searchParams: {
            expression: word.word,
          },
        })
        .json();

      console.log("DEBUG[1423]: response2=", response2);
    }
  };
  const dScanText = debounce(scanText, 100);

  document.addEventListener("mousemove", dScanText);
}
