import {
  createContext,
  useContext,
  ParentComponent,
  Setter,
  Accessor,
  createSignal,
} from "solid-js";
import useOption from "./option";
import { SearchWorkRequest, SortKind, Tag } from "./types";

const StoreContext = createContext<Store>();

export interface Store {
  setSearchText: Setter<string>;
  workPageMap: Map<string, number>; // key: workId, value: page
  sortKind: Accessor<SortKind>;
  setSortKind: Setter<SortKind>;
  isSortDesc: Accessor<boolean>;
  setIsSortDesc: Setter<boolean>;
  sortCol: () => "updated_at" | "name";
  searchRequest: () => SearchWorkRequest;
  isFilterArtist: Accessor<boolean>;
  setIsFilterArtist: Setter<boolean>;
  searchTags: Accessor<Tag[]>;
  setSearchTags: Setter<Tag[]>;
  dialogCount: Accessor<number>;
  incrementDialogCount: () => void;
  decrementDialogCount: () => void;
  refetch: () => void;
  setOffset: Setter<number>;
}

export const StoreProvider: ParentComponent = (props) => {
  const workPageMap = new Map(); // reaactive じゃなくていいためただの Map
  const {
    setText,
    sortKind,
    setSortKind,
    isSortDesc,
    setIsSortDesc,
    sortCol,
    request,
    isFilterArtist,
    setIsFilterArtist,
    tags,
    setTags,
    setOffset,
  } = useOption();
  const [dialogCount, setDialogCount] = createSignal(0);

  const store: Store = {
    setSearchText: setText,
    workPageMap,
    sortKind,
    setSortKind,
    isSortDesc,
    sortCol,
    setIsSortDesc,
    searchRequest: request,
    isFilterArtist,
    setIsFilterArtist,
    searchTags: tags,
    setSearchTags: setTags,
    dialogCount,
    incrementDialogCount: () => setDialogCount((prev) => prev + 1),
    decrementDialogCount: () => setDialogCount((prev) => prev - 1),
    refetch: () => {},
    setOffset,
  };

  return (
    <StoreContext.Provider value={store}>
      {props.children}
    </StoreContext.Provider>
  );
};

export const useStore = () => {
  return useContext(StoreContext);
};
