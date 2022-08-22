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
