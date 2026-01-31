import type { DictionaryEntry } from "@repo/server/types/db";
import type { Definition, DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";
import { ImageContent } from "./ImageContent";
import { createMemo, For, Show, type JSXElement } from "solid-js";
import { ShadowRoot } from "./ShadowRoot";
import { useResources } from "../util/resources";
import { uniqBy } from "es-toolkit";

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
    return definitionsTagNames().map((name) => resources.useDefinitionTags(name));
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

export function Popup(props: { expressions: string[] }) {
  const resources = useResources();
  const multipleDictionaryEntries = createMemo(() => {
    return props.expressions.map((expression) => resources.useDictionaryEntries(expression));
  });
  const dictionaryEntries = createMemo(() => {
    const multipleData = multipleDictionaryEntries().map(([data]) => data());
    const data = multipleData.flat().filter(Boolean);
    return uniqBy(data, (entry) => entry?.id);
  });

  return (
    <div
      class="p-2 w-[600px] h-[400px] overflow-scroll"
      style={{
        position: "absolute",
        top: 0,
        right: 0,
      }}
    >
      <For each={dictionaryEntries()}>
        {(entry) => (
          // TODO: fix hardcoded url
          <Show when={entry}>
            {(entry) => (
              <DefinirionEntry dictionaryEntry={entry()}>
                <ShadowRoot css={`http://localhost:45636/media/${entry().dictionaryId}/styles.css`}>
                  <For each={entry().definitions}>
                    {(definition) => <DefinitionRenderer definition={definition} />}
                  </For>
                </ShadowRoot>
              </DefinirionEntry>
            )}
          </Show>
        )}
      </For>
    </div>
  );
}
