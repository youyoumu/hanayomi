import { debounce } from "es-toolkit";
import type { Word } from "@repo/server/types/mecab-ipadic";

/* @refresh reload */
export function init() {
  const scanText = (e: MouseEvent) => {
    // console.log(e.clientX, e.clientY);
    const result = document.caretPositionFromPoint(e.clientX, e.clientY);
    if (result && result.offsetNode.nodeType === Node.TEXT_NODE) {
      const node = result.offsetNode as Text;
      const text = node.data;
      console.log(text);
    }
  };
  const dScanText = debounce(scanText, 100);

  document.addEventListener("mousemove", dScanText);
}
