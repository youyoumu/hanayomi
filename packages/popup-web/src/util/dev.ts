export function setupTailwind(shadow: ShadowRoot) {
  if (import.meta.env.DEV) {
    const syncStyles = () => {
      const tailwind = document.querySelector(
        'style[type="text/css"][data-vite-dev-id$="main.css"]',
      );
      if (tailwind) {
        let existingStyle = shadow.querySelector("style[data-shadow-sync]");

        if (!existingStyle) {
          existingStyle = document.createElement("style");
          existingStyle.setAttribute("data-shadow-sync", "true");
          shadow.appendChild(existingStyle);
        }
        existingStyle.textContent = tailwind.textContent;
      }
    };

    syncStyles();

    const observer = new MutationObserver(() => {
      syncStyles();
    });

    observer.observe(document.head, {
      childList: true,
      subtree: true,
      characterData: true,
    });
  }
}
