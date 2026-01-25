import type { ImageDefinition } from "@repo/server/types/dictionary-term-bank-v3";
import type { JSX } from "solid-js";

export function ImageContent(props: { imageDefinition: ImageDefinition }) {
  const {
    path,
    width,
    height,
    title,
    alt,
    description,
    pixelated,
    imageRendering,
    appearance,
    background,
    //TODO: implement
    collapsed: _collapsed,
    collapsible: _collapsible,
  } = props.imageDefinition;

  const imageStyle: JSX.CSSProperties = {
    "image-rendering": imageRendering || (pixelated ? "pixelated" : undefined),
    filter: appearance === "monochrome" ? "grayscale(100%)" : undefined,
    "background-color": background ? "transparent" : undefined,
  };

  const imageElement = (
    <img
      //TODO: served by server
      src={path}
      alt={alt || ""}
      title={title || undefined}
      width={width || undefined}
      height={height || undefined}
      style={imageStyle}
    />
  );

  if (description) {
    return (
      <figure>
        {imageElement}
        <figcaption>{description}</figcaption>
      </figure>
    );
  }

  return imageElement;
}

