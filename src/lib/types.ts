export interface Tag {
  id: string;
  name: string;
  updatedAt: string;
}

export interface Artist {
  id: string;
  name: string;
  updatedAt: string;
}

export interface Work {
  id: string;
  title: string;
  paths: string[];
  artist: Artist;
  tags: Tag[];
  updatedAt: string;
}

export interface WorkSummary {
  id: string;
  title: string;
  workListThumbnail: string;
  artistListThumbnail: string;
  artist: Artist;
}

export const SORT_KIND = ["追加日時", "作品名", "閲覧日時"] as const;
export const INITIAL_SELECT_SORT_OPTION = SORT_KIND[0];
export type SortKind = typeof SORT_KIND[number];

export const SORT_COLUMNS = ["updated_at", "title"] as const;
export type SortColumnKind = typeof SORT_COLUMNS[number];

export interface SearchWorkRequest {
  limit: number;
  offset: number;
  search: string;
  tags: string[];
  sortCol: SortColumnKind;
  sortDesc: boolean;
}

export interface SearchArtistRequest {
  limit: number;
  offset: number;
  search: string;
  sortCol: SortColumnKind;
  sortDesc: boolean;
}

export interface Suggest {
  artists: Artist[];
  tags: Tag[];
}

export const UNKNOWN_ARTIST_NAME = "Unknown Artist";
