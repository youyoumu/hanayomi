import type { DictionaryEntry } from "@repo/server/types/db";
import type { Definition, DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";
import { ImageContent } from "./ImageContent";
import {
  createEffect,
  createMemo,
  createSignal,
  For,
  onCleanup,
  onMount,
  Show,
  Suspense,
  type JSXElement,
} from "solid-js";
import { ShadowRoot } from "./ShadowRoot";
import { useResources } from "../util/resources";
import { debounce, uniq, uniqBy } from "es-toolkit";
import { LexemesProcessor } from "../util/lexeme";

function DefinitionRenderer(props: { definition: Definition }) {
  if (!props.definition) return null;
  if (typeof props.definition === "string") {
    return props.definition;
  }
  if (Array.isArray(props.definition)) {
    //TODO: implement deinflection
    return null;
  }
  const detailedDefinition = props.definition as DetailedDefinition;
  if (detailedDefinition.type === "text") {
    return detailedDefinition.text;
  }
  if (detailedDefinition.type === "image") {
    return <ImageContent imageDefinition={detailedDefinition} />;
  }
  if (detailedDefinition.type === "structured-content") {
    return <StructuredContentComponent structuredContent={detailedDefinition.content} />;
  }
  return null;
}

function DefinirionEntry(props: { dictionaryEntry: DictionaryEntry; children: JSXElement }) {
  const definitionsTagNames = () =>
    props.dictionaryEntry.definitionTags
      .split(" ")
      .map((tagName) => tagName.trim())
      .filter(Boolean);
  const resources = useResources();
  const multipleDefinitionTags = createMemo(() => {
    return definitionsTagNames().map((name) => resources.useDefinitionTags(() => name));
  });
  const definitionTags = createMemo(() => {
    const multipleData = multipleDefinitionTags().map(([data]) => data());
    const data = multipleData.flat().filter(Boolean);
    return uniqBy(data, (tag) => tag?.id);
  });

  return (
    <div class="flex flex-col gap-1">
      <div class="text-3xl">{props.dictionaryEntry.expression}</div>
      <div class="flex flex-wrap gap-1">
        <For each={definitionTags()}>
          {(tag) => (
            <Show when={tag}>
              {(tag) => <div class="badge badge-info badge-sm font-bold">{tag().name}</div>}
            </Show>
          )}
        </For>
      </div>
      <div>{props.children}</div>
    </div>
  );
}

export function Popup() {
  const resources = useResources();
  const [scanData, setScanData] = createSignal<{
    text: string;
    offset: number;
  }>();
  const text = () => scanData()?.text;
  const offset = () => scanData()?.offset;

  const [lexemes] = resources.useTokenize(text);
  const preprocess1 = createMemo(() => {
    const text_ = text();
    const offset_ = offset();
    if (!text_ || !offset_) return {};
    const lexemesProcessor = LexemesProcessor.new(lexemes());
    const lexeme = lexemesProcessor.getLexeme(offset_);
    const firstTokenLemma = lexemesProcessor.getFirstTokenLemma(lexeme);
    const wordClipped = lexemesProcessor.getWordClipped(offset_);

    const expressions = uniq([lexeme?.word, firstTokenLemma].filter(Boolean)) as string[];
    return { expressions, wordClipped };
  });

  const [lexemesClipped] = resources.useTokenize(() => preprocess1()?.wordClipped);
  const preprocess2 = createMemo(() => {
    const lexemesProcessor = LexemesProcessor.new(lexemesClipped());
    const lexeme = lexemesClipped()?.[0];
    const firstTokenLemma = lexemesProcessor.getFirstTokenLemma(lexeme);

    const expressions = uniq([firstTokenLemma].filter(Boolean)) as string[];
    return { expressions };
  });

  const expressions = createMemo(() => {
    const { expressions: expressions1 = [] } = preprocess1();
    const { expressions: expressions2 = [] } = preprocess2();
    return uniq([...expressions2, ...expressions1]);
  });

  const multipleDictionaryEntries = createMemo(() => {
    return expressions().map((expression) => resources.useDictionaryEntries(() => expression));
  });
  const dictionaryEntries = createMemo(() => {
    const multipleData = multipleDictionaryEntries()?.map(([data]) => data());
    const data = multipleData?.flat().filter(Boolean) as DictionaryEntry[];
    return uniqBy(data, (entry) => entry.id);
  });

  createEffect(() => {});

  onMount(() => {
    const scanText = async (e: MouseEvent) => {
      const result = document.caretPositionFromPoint(e.clientX - 5, e.clientY);
      if (result && result.offsetNode.nodeType === Node.TEXT_NODE) {
        const node = result.offsetNode as Text;
        const offset = result.offset;
        const text = node.data;
        setScanData({ text, offset });
      }
    };
    const dScanText = debounce(scanText, 100);
    document.addEventListener("mousemove", dScanText);
    onCleanup(() => {
      document.removeEventListener("mousemove", dScanText);
    });
  });

  return (
    <Suspense>
      <div
        class="p-2 w-[600px] h-[400px] overflow-scroll"
        style={{
          position: "absolute",
          top: 0,
          right: 0,
          display: dictionaryEntries()?.length > 0 ? "block" : "none",
        }}
      >
        <For each={dictionaryEntries()}>
          {(entry) => {
            return (
              // TODO: fix hardcoded url
              <DefinirionEntry dictionaryEntry={entry}>
                <ShadowRoot css={`http://localhost:45636/media/${entry.dictionaryId}/styles.css`}>
                  <For each={entry.definitions}>
                    {(definition) => <DefinitionRenderer definition={definition} />}
                  </For>
                </ShadowRoot>
              </DefinirionEntry>
            );
          }}
        </For>
      </div>
    </Suspense>
  );
}
