export interface Tag {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

export interface Artist {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

export interface Work {
  id: string;
  title: string;
  dirPath: string;
  artist: Artist;
  tags: Tag[];
  createdAt: string;
  updatedAt: string;
}
