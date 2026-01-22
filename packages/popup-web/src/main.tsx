/* @refresh reload */
import { render } from "solid-js/web";
import { Popup } from "./components/Popup";

export function init() {
  const root = document.getElementById("root");

  render(() => <Popup />, root!);
}
