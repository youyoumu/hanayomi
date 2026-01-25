import type { DictionaryEntry } from "@repo/server/types/db";
import type { Definition, DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";
import { ImageContent } from "./ImageContent";
import { For } from "solid-js";

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

export function Popup(props: { dictionaryEntries: DictionaryEntry[] }) {
  return (
    <div
      style={{
        height: "400px",
        width: "600px",
        overflow: "scroll",
      }}
    >
      <For each={props.dictionaryEntries}>
        {(entry) => (
          <For each={entry.definitions}>
            {(definition) => <DefinitionRenderer definition={definition} />}
          </For>
        )}
      </For>
    </div>
  );
}
