import { createQueryKeyStore } from "@lukemorales/query-key-factory";
import ky from "ky";

import type { Word } from "@repo/server/types/mecab-ipadic";
// import type { Word } from "@repo/server/types/dictionary-term-bank-v3";

const api = ky.create({
  prefixUrl: "http://localhost:45636",
  hooks: {
    afterResponse: [
      async (_, __, response) => {
        if (response.ok) {
          const clone = response.clone();
          const body = await clone.json();
          if (body && typeof body === "object" && "data" in body) {
            return response;
          }
        }
      },
    ],
  },
});

export const queries = createQueryKeyStore({
  tokenize: {
    detail: (sentence: string) => ({
      queryKey: [{ sentence }],
      queryFn: () => api.get<Word[]>(`tokenize`, { searchParams: { sentence } }).json(),
    }),
  },
  // dictionaryEntries: {
  //   detail: (expression: string) => ({
  //     queryKey: [{ expression }],
  //     queryFn: () =>
  //       api
  //         .get<DictionaryEntry[]>(`dictionary_entries/search`, { searchParams: { expression } })
  //         .json(),
  //   }),
  // },
});
