import { createQueryKeyStore } from "@lukemorales/query-key-factory";
import ky from "ky";

import type { Lexeme } from "@repo/server/types/mecab-ipadic";
import type { DefinitionTag, DictionaryEntry } from "@repo/server/types/db";

type Result<T> = {
  result: "success";
  data: T;
};

const api = ky.create({
  prefixUrl: "http://localhost:45636",
});

export const queries = createQueryKeyStore({
  tokenize: {
    detail: (sentence: string) => ({
      queryKey: [{ sentence }],
      queryFn: async () => {
        const result = await api
          .get<Result<Lexeme[]>>(`tokenize`, { searchParams: { sentence } })
          .json();
        return result.data;
      },
    }),
  },
  dictionaryEntries: {
    search: (expression: string) => ({
      queryKey: [{ expression }],
      queryFn: async () => {
        const result = await api
          .get<Result<DictionaryEntry[]>>(`dictionary_entries/search`, {
            searchParams: { expression },
          })
          .json();
        return result.data;
      },
    }),
  },
  definitionTags: {
    search: (name: string) => ({
      queryKey: [{ name }],
      queryFn: async () => {
        const result = await api
          .get<Result<DefinitionTag[]>>(`definition_tags/search`, {
            searchParams: { name },
          })
          .json();
        return result.data;
      },
    }),
  },
});
