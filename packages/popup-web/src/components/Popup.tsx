import type { DictionaryEntry } from "@repo/server/types/db";
import type { DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";
import { ImageContent } from "./ImageContent";

export function Popup(props: { dictionaryEntries: DictionaryEntry[] }) {
  const definitions = props.dictionaryEntries[0]?.definitions;
  if (!definitions) return null;
  const definition = definitions[0];
  if (!definition) return null;
  if (typeof definition === "string") {
    return definition;
  }
  if (Array.isArray(definition)) {
    //TODO: implement
    return null;
  }
  const detailedDefinition = definition as DetailedDefinition;
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
