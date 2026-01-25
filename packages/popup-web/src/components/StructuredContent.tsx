import type {
  StructuredContent,
  StructuredContentStyle,
  NumberOrString,
} from "@repo/server/types/dictionary-term-bank-v3";
import { For } from "solid-js";
import type { JSX } from "solid-js";

export function StructuredContentComponent(props: {
  structuredContent: StructuredContent | null | undefined;
}) {
  if (!props.structuredContent) return null;
  if (typeof props.structuredContent === "string") {
    return props.structuredContent;
  }
  if (Array.isArray(props.structuredContent)) {
    return (
      <For each={props.structuredContent}>
        {(item) => <StructuredContentComponent structuredContent={item} />}
      </For>
    );
  }

  const { tag } = props.structuredContent;
  const data = "data" in props.structuredContent ? props.structuredContent.data : undefined;
  const dataAttribute = createDataAttribute(data);
  const styleAttribute = createStyleAttribute(
    "style" in props.structuredContent ? props.structuredContent.style : undefined,
  );
  const commonAttributes = { ...dataAttribute, ...styleAttribute };

  switch (tag) {
    case "br":
      return <br {...dataAttribute} />;

    case "ruby":
      return (
        <ruby {...dataAttribute}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </ruby>
      );

    case "rt":
      return (
        <rt {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </rt>
      );

    case "rp":
      return (
        <rp {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </rp>
      );

    case "table":
      return (
        <table {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </table>
      );

    case "thead":
      return (
        <thead {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </thead>
      );

    case "tbody":
      return (
        <tbody {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </tbody>
      );

    case "tfoot":
      return (
        <tfoot {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </tfoot>
      );

    case "tr":
      return (
        <tr {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </tr>
      );

    case "td":
      return (
        <td {...commonAttributes} lang={props.structuredContent.lang || undefined}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </td>
      );

    case "th":
      return (
        <th {...commonAttributes} lang={props.structuredContent.lang || undefined}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </th>
      );

    case "span":
      return (
        <span {...commonAttributes}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </span>
      );

    case "div":
      return (
        <div
          {...commonAttributes}
          title={props.structuredContent.title || undefined}
          lang={props.structuredContent.lang || undefined}
        >
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </div>
      );

    case "ol":
      return (
        <ol
          {...commonAttributes}
          title={props.structuredContent.title || undefined}
          lang={props.structuredContent.lang || undefined}
        >
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </ol>
      );

    case "ul":
      return (
        <ul
          {...commonAttributes}
          title={props.structuredContent.title || undefined}
          lang={props.structuredContent.lang || undefined}
        >
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </ul>
      );

    case "li":
      return (
        <li
          {...commonAttributes}
          title={props.structuredContent.title || undefined}
          lang={props.structuredContent.lang || undefined}
        >
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </li>
      );

    case "details":
      return (
        <details
          {...commonAttributes}
          title={props.structuredContent.title || undefined}
          open={props.structuredContent.open || undefined}
          lang={props.structuredContent.lang || undefined}
        >
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </details>
      );

    case "summary":
      return (
        <summary
          {...commonAttributes}
          title={props.structuredContent.title || undefined}
          lang={props.structuredContent.lang || undefined}
        >
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </summary>
      );

    case "img":
      return (
        <img
          {...dataAttribute}
          src={props.structuredContent.path}
          alt={props.structuredContent.alt || ""}
          title={props.structuredContent.title || undefined}
          width={props.structuredContent.width || undefined}
          height={props.structuredContent.height || undefined}
          style={
            {
              "image-rendering": props.structuredContent.imageRendering || undefined,
              "vertical-align": props.structuredContent.verticalAlign || undefined,
              border: props.structuredContent.border || undefined,
              "border-radius": props.structuredContent.borderRadius || undefined,
            } as JSX.CSSProperties
          }
        />
      );

    case "a":
      return (
        <a href={props.structuredContent.href} lang={props.structuredContent.lang || undefined}>
          <StructuredContentComponent structuredContent={props.structuredContent.content} />
        </a>
      );

    default:
      // Fallback for unknown tags - use type assertion to handle unknown tag types
      const unknownStructuredContent = props.structuredContent as any;
      return (
        <span {...commonAttributes}>
          <StructuredContentComponent structuredContent={unknownStructuredContent.content} />
        </span>
      );
  }
}

function createDataAttribute(data: Record<string, string> | null | undefined) {
  if (!data) return {};
  const result: Record<string, string> = {};
  for (const [key, value] of Object.entries(data)) {
    result[`data-sc-${key}`] = value;
  }
  return result;
}

function createStyleAttribute(style: StructuredContentStyle | null | undefined) {
  if (!style) return {};

  const styleObj: JSX.CSSProperties = {};

  // Map all style properties from the StructuredContentStyle type
  if (style.fontStyle) styleObj["font-style"] = style.fontStyle;
  if (style.fontWeight) styleObj["font-weight"] = style.fontWeight;
  if (style.fontSize) styleObj["font-size"] = style.fontSize;
  if (style.color) styleObj.color = style.color;
  if (style.background) styleObj.background = style.background;
  if (style.backgroundColor) styleObj["background-color"] = style.backgroundColor;
  if (style.textDecorationLine) {
    styleObj["text-decoration-line"] = Array.isArray(style.textDecorationLine)
      ? style.textDecorationLine.join(" ")
      : style.textDecorationLine;
  }
  if (style.textDecorationStyle) styleObj["text-decoration-style"] = style.textDecorationStyle;
  if (style.textDecorationColor) styleObj["text-decoration-color"] = style.textDecorationColor;
  if (style.borderColor) styleObj["border-color"] = style.borderColor;
  if (style.borderStyle) styleObj["border-style"] = style.borderStyle;
  if (style.borderRadius) styleObj["border-radius"] = style.borderRadius;
  if (style.borderWidth) styleObj["border-width"] = style.borderWidth;
  if (style.clipPath) styleObj["clip-path"] = style.clipPath;
  if (style.verticalAlign) styleObj["vertical-align"] = style.verticalAlign;
  if (style.textAlign && style.textAlign !== "justify-all")
    styleObj["text-align"] = style.textAlign;
  if (style.textEmphasis) styleObj["text-emphasis"] = style.textEmphasis;
  if (style.textShadow) styleObj["text-shadow"] = style.textShadow;
  if (style.margin) styleObj.margin = style.margin;

  const convertNumberOrString = (value: NumberOrString | null | undefined): string | undefined => {
    if (!value) return undefined;
    if (typeof value === "string") return value;
    if (typeof value === "number") return value.toString();
    return undefined;
  };

  if (style.marginTop) {
    const converted = convertNumberOrString(style.marginTop);
    if (converted !== undefined) styleObj["margin-top"] = converted;
  }
  if (style.marginLeft) {
    const converted = convertNumberOrString(style.marginLeft);
    if (converted !== undefined) styleObj["margin-left"] = converted;
  }
  if (style.marginRight) {
    const converted = convertNumberOrString(style.marginRight);
    if (converted !== undefined) styleObj["margin-right"] = converted;
  }
  if (style.marginBottom) {
    const converted = convertNumberOrString(style.marginBottom);
    if (converted !== undefined) styleObj["margin-bottom"] = converted;
  }

  if (style.padding) styleObj.padding = style.padding;
  if (style.paddingTop) styleObj["padding-top"] = style.paddingTop;
  if (style.paddingLeft) styleObj["padding-left"] = style.paddingLeft;
  if (style.paddingRight) styleObj["padding-right"] = style.paddingRight;
  if (style.paddingBottom) styleObj["padding-bottom"] = style.paddingBottom;
  if (style.wordBreak) styleObj["word-break"] = style.wordBreak;
  if (style.whiteSpace) styleObj["white-space"] = style.whiteSpace;
  if (style.cursor) styleObj.cursor = style.cursor;
  if (style.listStyleType) styleObj["list-style-type"] = style.listStyleType;

  return { style: styleObj };
}
