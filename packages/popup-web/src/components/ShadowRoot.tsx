import type { JSXElement } from "solid-js";
import { Portal } from "solid-js/web";

export function ShadowRoot(props: { children: JSXElement; css: string }) {
  let div: HTMLDivElement;

  return (
    <div ref={div!}>
      <Portal mount={div!} useShadow={true} ref={(x) => (x.style.display = "contents")}>
        <link rel="stylesheet" href={props.css} />
        {props.children}
      </Portal>
    </div>
  );
}
