import type { DefinitionTag, DictionaryEntry } from "@repo/server/types/db";
import { createContextProvider } from "@solid-primitives/context";
import { makeCache } from "@solid-primitives/resource";
import ky from "ky";
import { createResource } from "solid-js";

type Result<T> = {
  result: "success";
  data: T;
};

const api = ky.create({
  prefixUrl: "http://localhost:45636",
});

const [ResourcesContextProvider, useResources_] = createContextProvider(() => {
  const definitionTagsFc = makeCache(async ([_key, name]: ["definitionTags", string]) => {
    const result = await api
      .get<Result<DefinitionTag[]>>(`definition_tags/search`, {
        searchParams: { name },
      })
      .json();
    return result.data;
  });
  const useDefinitionTags = (name: string) => {
    return createResource(["definitionTags", name], definitionTagsFc[0]);
  };

  const dictionaryEntriesCh = makeCache(
    async ([_key, expression]: ["dictionaryEntries", string]) => {
      const result = await api
        .get<Result<DictionaryEntry[]>>(`dictionary_entries/search`, {
          searchParams: { expression },
        })
        .json();
      return result.data;
    },
  );
  const useDictionaryEntries = (expression: string) => {
    return createResource(["dictionaryEntries", expression], dictionaryEntriesCh[0]);
  };

  return {
    useDefinitionTags,
    useDictionaryEntries,
  };
});

const useResources = () => {
  const resources = useResources_();
  if (!resources) throw new Error("Missing ResourcesContextProvider");
  return resources;
};

export { ResourcesContextProvider, useResources };
