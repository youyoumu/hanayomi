import type { DefinitionTag, DictionaryEntry } from "@repo/server/types/db";
import type { Lexeme } from "@repo/server/types/mecab-ipadic";
import { createContextProvider } from "@solid-primitives/context";
import { makeCache } from "@solid-primitives/resource";
import ky from "ky";
import { createMemo, createResource } from "solid-js";

type Result<T> = {
  result: "success";
  data: T;
};

const api = ky.create({
  prefixUrl: "http://localhost:45636",
});

const [ResourcesContextProvider, useResources_] = createContextProvider(() => {
  type DefinitionTagsKey = ["definitionTags", string];
  const [definitionTagsFc] = makeCache(async ([_key, name]: DefinitionTagsKey) => {
    return api
      .get<Result<DefinitionTag[]>>(`definition_tags/search`, {
        searchParams: { name },
      })
      .json()
      .then((result) => result.data);
  });
  const useDefinitionTags = (name: () => string | undefined) => {
    const param = createMemo(() => {
      const name_ = name();
      return name_ ? (["definitionTags", name_] satisfies DefinitionTagsKey) : undefined;
    });
    return createResource(param, definitionTagsFc);
  };

  type DictionaryEntriesKey = ["dictionaryEntries", string];
  const [dictionaryEntriesFc] = makeCache(async ([_key, expression]: DictionaryEntriesKey) => {
    return api
      .get<Result<DictionaryEntry[]>>(`dictionary_entries/search`, {
        searchParams: { expression },
      })
      .json()
      .then((result) => result.data);
  });
  const useDictionaryEntries = (expression: () => string | undefined) => {
    const param = createMemo(() => {
      const expression_ = expression();
      return expression_
        ? (["dictionaryEntries", expression_] satisfies DictionaryEntriesKey)
        : undefined;
    });
    return createResource(param, dictionaryEntriesFc);
  };

  type TokenizeKey = ["tokenize", string];
  const [tokenizeFc] = makeCache(async ([_key, sentence]: TokenizeKey) => {
    return api
      .get<Result<Lexeme[]>>(`tokenize`, { searchParams: { sentence } })
      .json()
      .then((result) => result.data);
  });
  const useTokenize = (sentence: () => string | undefined) => {
    const param = createMemo(() => {
      const sentence_ = sentence();
      return sentence_ ? (["tokenize", sentence_] satisfies TokenizeKey) : undefined;
    });
    return createResource(param, tokenizeFc);
  };

  return {
    useDefinitionTags,
    useDictionaryEntries,
    useTokenize,
  };
});

const useResources = () => {
  const resources = useResources_();
  if (!resources) throw new Error("Missing ResourcesContextProvider");
  return resources;
};

export { ResourcesContextProvider, useResources };
