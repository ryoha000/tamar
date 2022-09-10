import { invoke } from "@tauri-apps/api";
import { Usages } from "../components/TopPage/use/dirUsage";
import { DirPathInfo } from "../components/TopPage/use/exploreDir";
import {
  Artist,
  SearchArtistRequest,
  SearchWorkRequest,
  Suggest,
  Tag,
  Work,
  WorkSummary,
} from "./types";

export const commandImportDirectory = async (payload: {
  dirPathInfos: DirPathInfo[];
  usages: Usages;
}) => {
  await invoke<null>("import_directory", payload);
};

export const commandSearchWork = async (payload: SearchWorkRequest) => {
  return await invoke<WorkSummary[]>("search_work", {
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
  return await invoke<string[]>("search_around_title_work", {
    limit: payload.limit,
    isBefore: payload.isBefore,
    title: payload.title,
  });
};

export const commandSearchAroundUpdatedAtWork = async (payload: {
  limit: number;
  isBefore: boolean;
  updated_at: string;
}) => {
  return await invoke<string[]>("search_around_updated_at_work", {
    limit: payload.limit,
    isBefore: payload.isBefore,
    updatedAt: payload.updated_at,
  });
};

export const commandGetWork = async (id: String) => {
  return await invoke<Work>("get_work", { id });
};

export const commandSelectWorkByArtist = async (artistId: String) => {
  return await invoke<WorkSummary[]>("select_work_by_artist", { artistId });
};

export const commandSearchArtist = async (payload: SearchArtistRequest) => {
  return await invoke<Artist[]>("search_artist", {
    limit: payload.limit,
    offset: payload.offset,
    search: payload.search,
    sortCol: payload.sortCol,
    sortDesc: payload.sortDesc,
  });
};

export const commandGetArtist = async (artist_id: string) => {
  return await invoke<Artist>("get_artist", {
    id: artist_id,
  });
};

export const commandGetInitialSuggest = async (limit: number) => {
  return await invoke<Suggest>("get_initial_suggest", { limit });
};

export const commandGetSuggest = async (text: string) => {
  return await invoke<Suggest>("get_suggest", { text });
};

export const commandGetTagSuggest = async (text: string) => {
  return await invoke<Tag[]>("get_tag_suggest", { text });
};

export const commandUseSuggest = async (payload: {
  valueId: string;
  valueType: number;
}) => {
  return await invoke<null>("use_suggest", { ...payload });
};

export const commandSelectTag = async (limit: number) => {
  return await invoke<Tag[]>("select_tag", { limit });
};

export const commandAttachTag = async (payload: {
  workId: string;
  tagId: string;
}) => {
  return await invoke<void>("attach_tag", payload);
};

export const commandDetachTag = async (payload: {
  workId: string;
  tagId: string;
}) => {
  return await invoke<void>("detach_tag", payload);
};

export const commandAttachTagByName = async (payload: {
  workId: string;
  name: string;
}) => {
  return await invoke<void>("attach_tag_by_name", payload);
};

export const commandUpdateWorkTitle = async (payload: {
  id: string;
  title: string;
}) => {
  return await invoke<void>("update_work_title", payload);
};

export const commandUpdateWorkArtist = async (payload: {
  id: string;
  name: string;
}) => {
  return await invoke<void>("update_work_artist", payload);
};

export const commandDeleteWork = async (id: string) => {
  return await invoke<void>("delete_work", { id });
};

export const commandSelectArtistByName = async (name: string) => {
  return await invoke<Artist[]>("select_artist_by_name", { name });
};

export const commandRotateWorkFile = async (file: string) => {
  return await invoke<void>("rotate_work_file", { file });
};

export const commandDeleteWorkFile = async (file: string) => {
  return await invoke<void>("delete_work_file", { file });
};

export const commandUpdateArtistName = async (payload: {
  id: string;
  name: string;
}) => {
  return await invoke<void>("update_artist_name", {
    id: payload.id,
    name: payload.name,
  });
};

export const commandImportFile = async (payload: {
  artistName: string;
  filePaths: string[];
}) => {
  return await invoke<void>("import_file", payload);
};

export const commandViewWork = async (workId: string) => {
  return await invoke<void>("view_work", { workId });
};

export const commandDeleteAllData = async () => {
  return await invoke<void>("delete_all_data", {});
};
