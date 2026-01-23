import type { DictionaryEntry } from "@repo/server/types/db";
import type {
  StructuredContent,
  DetailedDefinition,
} from "@repo/server/types/dictionary-term-bank-v3";

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
    //TODO: implement
    return null;
  }
  if (detailedDefinition.type === "structured-content") {
    const structuredContent = detailedDefinition.structuredContent.content;
  }

  return (
    <div
      style={{
        width: "320px",
        height: "240px",
        "background-color": "navy",
        color: "white",
      }}
    >
      Hello world
    </div>
  );
}
