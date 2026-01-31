/* @refresh reload */
import { debounce, uniq } from "es-toolkit";
import { render } from "solid-js/web";
import { Popup } from "./components/Popup";
import "./styles/main.css";
import { setupTailwind } from "./util/dev";
import { LexemesProcessor } from "./util/lexeme";
import ky from "ky";
import type { Lexeme } from "@repo/server/types/mecab-ipadic";

type Result<T> = {
  result: "success";
  data: T;
};

const api = ky.create({
  prefixUrl: "http://localhost:45636",
});

async function getExpressions({ text, offset }: { text: string; offset: number }) {
  const lexemes = (
    await api.get<Result<Lexeme[]>>(`tokenize`, { searchParams: { sentence: text } }).json()
  ).data;

  const lexemesProcessor = LexemesProcessor.new(lexemes);
  const lexeme = lexemesProcessor.getLexeme(offset);
  if (!lexeme) return [];

  const firstTokenLemma = lexemesProcessor.getFirstTokenLemma(lexeme);

  const wordClipped = lexemesProcessor.getWordClipped(offset);
  const lexemesClipped = wordClipped
    ? (
        await api
          .get<Result<Lexeme[]>>(`tokenize`, { searchParams: { sentence: wordClipped } })
          .json()
      ).data
    : [];
  const lexemeClipped = lexemesClipped[0];
  const lexemesClippedProcessor = LexemesProcessor.new(lexemesClipped);
  const firstTokenLemmaClipped = lexemeClipped
    ? lexemesClippedProcessor.getFirstTokenLemma(lexemeClipped)
    : null;

  const expressions = uniq(
    [firstTokenLemmaClipped, lexeme.word, firstTokenLemma].filter(Boolean),
  ) as string[];

  return expressions;
}

export function init() {
  const pupup = document.createElement("div");
  const root = document.createElement("div");
  const shadow = pupup.attachShadow({ mode: "closed" });
  setupTailwind(shadow);
  shadow.appendChild(root);
  document.body.appendChild(pupup);

  const scanText = async (e: MouseEvent) => {
    // console.log(e.clientX, e.clientY);
    const result = document.caretPositionFromPoint(e.clientX - 5, e.clientY);
    if (result && result.offsetNode.nodeType === Node.TEXT_NODE) {
      const node = result.offsetNode as Text;
      const offset = result.offset;
      const text = node.data;
      const expressions = await getExpressions({ text, offset });

      root.innerHTML = "";
      render(() => <Popup expressions={expressions} />, root);
    }
  };
  const dScanText = debounce(scanText, 100);

  document.addEventListener("mousemove", dScanText);
}
