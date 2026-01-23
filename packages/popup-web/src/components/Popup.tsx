import type { DictionaryEntry } from "@repo/server/types/db";
import type { DetailedDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import { StructuredContentComponent } from "./StructuredContent";

export function Popup(props: { dictionaryEntries: DictionaryEntry[] }) {
  const definitions = props.dictionaryEntries[0]?.definitions;
  console.log("DEBUG[1433]: props.dictionaryEntries=", props.dictionaryEntries);
  console.log("DEBUG[1432]: definitions=", definitions);
  if (!definitions) return null;
  const definition = definitions[0];
  if (!definition) return null;
  console.log("DEBUG[1430]: definition=", definition);
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
    return <StructuredContentComponent structuredContent={detailedDefinition.content} />;
  }
  return null;
}
