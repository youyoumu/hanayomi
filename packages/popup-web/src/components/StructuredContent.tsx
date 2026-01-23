import type { StructuredContent } from "@repo/server/types/dictionary-term-bank-v3";
import { createComponent, For } from "solid-js";

export function StructuredContent(props: {
  structuredContent: StructuredContent | null | undefined;
}) {
  if (!props.structuredContent) return null;
  if (typeof props.structuredContent === "string") {
    return props.structuredContent;
  }
  if (Array.isArray(props.structuredContent)) {
    return (
      <For each={props.structuredContent}>
        {(item) => <StructuredContent structuredContent={item} />}
      </For>
    );
  }

  if (props.structuredContent.tag === "br") {
    const data = props.structuredContent.data;
    const dataAttribute = createDataAttribute(data);
    return <br {...dataAttribute} />;
  }

  if (props.structuredContent.tag === "ruby") {
    const data = props.structuredContent.data;
    const dataAttribute = createDataAttribute(data);
    return (
      <ruby {...dataAttribute}>
        <StructuredContent structuredContent={props.structuredContent.content} />
      </ruby>
    );
  }
}

function createDataAttribute(data: Record<string, string> | null | undefined) {
  if (!data) return {};
  const result: Record<string, string> = {};
  for (const [key, value] of Object.entries(data)) {
    result[`data-${key}`] = value;
  }
}
