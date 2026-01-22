import { join } from "node:path";
import fs from "node:fs/promises";
import { execa } from "execa";

const ROOT = join(import.meta.dirname, "../");
export const PATH = {
  ROOT: join(import.meta.dirname, "../"),
  VIBRATO_MODELS: join(ROOT, ".vibrato_models"),
};

export const MODELS = {
  IPADIC_MECAB: {
    url: "https://github.com/daac-tools/vibrato/releases/download/v0.5.0/ipadic-mecab-2_7_0.tar.xz",
    filename: "ipadic-mecab-2_7_0.tar.xz",
    get filePath() {
      return join(PATH.VIBRATO_MODELS, this.filename);
    },
    get extractPath() {
      return PATH.VIBRATO_MODELS;
    },
  },
} as const;

export const download = async (url: string, dest: string) => {
  await execa`wget -O ${dest} ${url}`;
};

export async function exists(path: string) {
  try {
    await fs.access(path);
    return true;
  } catch {
    return false;
  }
}

export async function ensureAllDirExists() {
  await fs.mkdir(PATH.VIBRATO_MODELS, { recursive: true });
  await fs.mkdir(MODELS.IPADIC_MECAB.extractPath, { recursive: true });
}
