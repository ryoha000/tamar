import { invoke } from "@tauri-apps/api";
import { Usages } from "../components/TopPage/use/dirUsage";
import { DirPathInfo } from "../components/TopPage/use/exploreDir";
import { Work } from "./types";

export const commandImportDirectory = async (
  dirPathInfos: DirPathInfo[],
  usages: Usages
) => {
  await invoke<null>("import_directory", { dirPathInfos, usages });
};

export const SORT_COLUMNS = ["updated_at", "title"] as const;
export type SortColumnKind = typeof SORT_COLUMNS[number];

export const commandSearchWork = async (payload: {
  limit: number;
  offset: number;
  search: string;
  tags: string[];
  sortCol: string;
  sortDesc: boolean;
}) => {
  return await invoke<Work[]>("search_work", {
    limit: payload.limit,
    offset: payload.offset,
    search: payload.search,
    tags: payload.tags,
    sortCol: payload.sortCol,
    sortDesc: payload.sortDesc,
  });
};

export const commandSearchAroundTitleWork = async (payload: {
  limit: number;
  isBefore: boolean;
  title: string;
}) => {
  return await invoke<Work[]>("search_around_title_work", {
    limit: payload.limit,
    isBefore: payload.isBefore,
    title: payload.title,
  });
};

export const commandGetWork = async (id: String) => {
  return await invoke<Work>("get_work", { id });
};
