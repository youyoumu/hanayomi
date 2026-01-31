import type { DictionaryEntry, DefinitionTag } from "@repo/server/types/db";
import type { Definition, DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";
import { ImageContent } from "./ImageContent";
import { For, Show, type JSXElement } from "solid-js";
import { ShadowRoot } from "./ShadowRoot";
import { useQueries } from "@tanstack/solid-query";
import { queries } from "../util/queryKeyFactory";

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
  const definitionsTags = props.dictionaryEntry.definitionTags
    .split(" ")
    .map((tagName) => tagName.trim())
    .filter(Boolean);
  const query = useQueries(() => ({
    queries: definitionsTags.map((tagName) => ({
      ...queries.definitionTags.search(tagName),
      select: (data: DefinitionTag[]) => {
        return data.find((tag) => tag.dictionaryId === props.dictionaryEntry.dictionaryId);
      },
    })),
  }));

  return (
    <div class="flex flex-col gap-1">
      <div class="text-3xl">{props.dictionaryEntry.expression}</div>
      <div class="flex flex-wrap gap-1">
        <For each={query}>
          {(query) => {
            return (
              <Show when={query.data}>
                <div class="badge badge-info badge-sm font-bold">{query.data?.name}</div>
              </Show>
            );
          }}
        </For>
      </div>
      <div>{props.children}</div>
    </div>
  );
}

export function Popup(props: { expressions: string[] }) {
  const query = useQueries(() => ({
    queries: props.expressions.map((expression) => ({
      ...queries.dictionaryEntries.search(expression),
    })),
  }));

  return (
    <div
      class="p-2 w-[600px] h-[400px] overflow-scroll"
      style={{
        position: "absolute",
        top: 0,
        right: 0,
      }}
    >
      <For each={query}>
        {(query) => (
          <For each={query.data}>
            {(entry) => (
              //  TODO: fix hardcoded url
              <DefinirionEntry dictionaryEntry={entry}>
                <ShadowRoot css={`http://localhost:45636/media/${entry.dictionaryId}/styles.css`}>
                  <For each={entry.definitions}>
                    {(definition) => <DefinitionRenderer definition={definition} />}
                  </For>
                </ShadowRoot>
              </DefinirionEntry>
            )}
          </For>
        )}
      </For>
    </div>
  );
}
