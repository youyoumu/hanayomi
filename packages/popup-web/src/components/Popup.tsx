import type { DictionaryEntry } from "@repo/server/types/db";
import type { Definition, DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";
import { ImageContent } from "./ImageContent";
import { For, type JSXElement } from "solid-js";
import { ShadowRoot } from "./ShadowRoot";
import { useDefinitionTags, useDictionaryEntries } from "../util/resources";

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
  const [definitionTags] = useDefinitionTags(definitionsTagNames());

  return (
    <div class="flex flex-col gap-1">
      <div class="text-3xl">{props.dictionaryEntry.expression}</div>
      <div class="flex flex-wrap gap-1">
        <For each={definitionTags()}>
          {(data) => {
            return <div class="badge badge-info badge-sm font-bold">{data.name}</div>;
          }}
        </For>
      </div>
      <div>{props.children}</div>
    </div>
  );
}

export function Popup(props: { expressions: string[] }) {
  const [dictionaryEntries] = useDictionaryEntries(props.expressions);

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
    </div>
  );
}
