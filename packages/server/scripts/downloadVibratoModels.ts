import { PATH, MODELS, download, ensureAllDirExists, exists } from "./utils.ts";
import { execa } from "execa";

class Script {
  async run(): Promise<void> {
    ensureAllDirExists();
    console.log("Downloading Vibrato models...");

    for (const model of Object.values(MODELS)) {
      const { filePath, url, filename, extractPath } = model;
      if (await exists(filePath)) {
        console.log(`${filename} already exists, skipping...`);
        continue;
      }
      // File doesn't exist, proceed with download
      await download(url, MODELS.IPADIC_MECAB.filePath);
      console.log(`Downloaded ${MODELS.IPADIC_MECAB.filename} to ${PATH.VIBRATO_MODELS}`);

      console.log(`Extracting ${filename}...`);
      await execa`tar -xf ${filePath} -C ${extractPath}`;
      console.log(`Extracted to ${extractPath}`);
    }
  }
}

const script = new Script();
script.run();
