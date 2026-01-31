import type { DefinitionTag, DictionaryEntry } from "@repo/server/types/db";
import { uniqBy } from "es-toolkit";
import ky from "ky";
import { createResource } from "solid-js";

type Result<T> = {
  result: "success";
  data: T;
};

const api = ky.create({
  prefixUrl: "http://localhost:45636",
});

export function useDefinitionTags(definitionTagNames: string[]) {
  return createResource(definitionTagNames, async (definitionTags) => {
    const dataP = definitionTags.map(async (tagName) => {
      const result = await api
        .get<Result<DefinitionTag[]>>(`definition_tags/search`, {
          searchParams: { name: tagName },
        })
        .json();
      return result.data;
    });
    const data = (await Promise.all(dataP)).flat();
    return uniqBy(data, (tag) => tag.id);
  });
}

export function useDictionaryEntries(expressions: string[]) {
  return createResource(expressions, async (expressions) => {
    const dataP = expressions.map(async (expression) => {
      const result = await api
        .get<Result<DictionaryEntry[]>>(`dictionary_entries/search`, {
          searchParams: { expression },
        })
        .json();
      return result.data;
    });
    const data = (await Promise.all(dataP)).flat();
    return uniqBy(data, (entry) => entry.id);
  });
}
