import {
  createContext,
  useContext,
  ParentComponent,
  Setter,
  Accessor,
} from "solid-js";
import useOption from "./option";
import { SearchWorkRequest, SortKind } from "./types";

const StoreContext = createContext<Store>();

export interface Store {
  setSearchText: Setter<string>;
  workPageMap: Map<string, number>; // key: workId, value: page
  sortKind: Accessor<SortKind>;
  setSortKind: Setter<SortKind>;
  isSortDesc: Accessor<boolean>;
  setIsSortDesc: Setter<boolean>;
  sortCol: () => "updated_at" | "title";
  searchRequest: () => SearchWorkRequest;
  isFilterArtist: Accessor<boolean>;
  setIsFilterArtist: Setter<boolean>;
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
  } = useOption();

  const store = {
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
