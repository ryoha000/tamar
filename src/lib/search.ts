import { createResource, createSignal } from "solid-js";
import { commandSearchWork, SortColumnKind } from "./commands";
import { SortKind } from "../components/TopPage/SortSelect";

const SEARCH_LIMIT = 30;

const useSearch = () => {
  const [offset, setOffset] = createSignal(0);
  const [text, setText] = createSignal("");
  const [tags, setTags] = createSignal<string[]>([]);
  const [sortKind, setSortKind] = createSignal<SortKind>("追加日時");
  const [isSortDesc, setIsSortDesc] = createSignal(true);

  const sortCol = (): SortColumnKind => {
    const kind = sortKind();
    switch (kind) {
      case "作品名":
        return "title";
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

  const payload = () => ({
    limit: SEARCH_LIMIT,
    offset: offset(),
    search: text(),
    tags: tags(),
    sortCol: sortCol(),
    sortDesc: isSortDesc(),
  });

  const [works, { refetch }] = createResource(payload, commandSearchWork, {
    initialValue: [],
  });

  // TODO: スクロールでオフセット増やす処理

  return { works, setText };
};

export default useSearch;
