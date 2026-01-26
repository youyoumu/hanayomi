/* @refresh reload */
import { debounce, uniq } from "es-toolkit";
import { queries } from "./util/queryKeyFactory";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { render } from "solid-js/web";
import { Popup } from "./components/Popup";
import "./styles/main.css";
import { setupTailwind } from "./util/dev";
import { LexemesProcessor } from "./util/lexeme";

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

      const lexemes = await queryClient.fetchQuery({
        ...queries.tokenize.detail(text),
      });

      const lexemesProcessor = LexemesProcessor.new(lexemes);
      console.log("DEBUG[1458]: lexemes=", lexemes);
      const lexemeIndex = lexemesProcessor.getLexemeIndex(offset);
      const lexeme = lexemes[lexemeIndex];
      if (!lexeme) return;
      const firstTokenLemma = lexemesProcessor.getFirstTokenLemma(lexeme);
      const expressions = uniq([lexeme.word, firstTokenLemma].filter(Boolean)) as string[];

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
