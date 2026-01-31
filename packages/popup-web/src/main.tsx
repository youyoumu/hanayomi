/* @refresh reload */
import { render } from "solid-js/web";
import { Popup } from "./components/Popup";
import "./styles/main.css";
import { setupTailwind } from "./util/dev";
import { ResourcesContextProvider } from "./util/resources";
import { MultiProvider } from "@solid-primitives/context";

export function init() {
  const pupup = document.createElement("div");
  const root = document.createElement("div");
  const shadow = pupup.attachShadow({ mode: "closed" });
  setupTailwind(shadow);
  shadow.appendChild(root);
  document.body.appendChild(pupup);

  render(
    () => (
      <MultiProvider
        values={[
          [ResourcesContextProvider, undefined],
          // more providers here
        ]}
      >
        <Popup />
      </MultiProvider>
    ),
    root,
  );
}
