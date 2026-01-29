import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  root: "./src/sample-page/sample2",
  build: {
    lib: {
      entry: path.resolve(import.meta.dirname, "src/main.tsx"),
      name: "hanayomi",
      fileName: "hanayomi",
      formats: ["es"],
    },
    copyPublicDir: false,
    cssCodeSplit: false,
    cssMinify: false,
    minify: false,
    rolldownOptions: {
      output: {
        minify: false,
      },
    },
    outDir: "../../../dist",
    emptyOutDir: true,
  },
});
