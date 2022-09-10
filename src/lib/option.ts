import { createSignal } from "solid-js";
import {
  SortKind,
  INITIAL_SELECT_SORT_OPTION,
  SortColumnKind,
  Tag,
  SearchWorkRequest,
} from "./types";

export const SEARCH_LIMIT = 30;

const useOption = () => {
  const [offset, setOffset] = createSignal(0);
  const [text, setText] = createSignal("");
  const [tags, setTags] = createSignal<Tag[]>([]);
  const [sortKind, setSortKind] = createSignal<SortKind>(
    INITIAL_SELECT_SORT_OPTION
  );
  const [isSortDesc, setIsSortDesc] = createSignal(true);
  const [isFilterArtist, setIsFilterArtist] = createSignal(true);

  const sortCol = (): SortColumnKind => {
    const kind = sortKind();
    switch (kind) {
      case "名前":
        return "name";
      case "追加日時":
        return "updated_at";
      case "閲覧日時":
        // TODO: 後で考える
        return "updated_at";
      default:
        const _exhaustType: never = kind;
        throw Error(`unknown sortKind. sortKind: ${kind}`);
    }
  };

  const tagIds = () => tags().map((v) => v.id);
  const request = (): SearchWorkRequest => ({
    limit: SEARCH_LIMIT,
    offset: offset(),
    search: text(),
    tags: tagIds(),
    sortCol: sortCol(),
    sortDesc: isSortDesc(),
  });

  return {
    request,
    setText,
    sortKind,
    setSortKind,
    isSortDesc,
    setIsSortDesc,
    sortCol,
    isFilterArtist,
    setIsFilterArtist,
    tags,
    setTags,
    setOffset,
  };
};

export default useOption;
