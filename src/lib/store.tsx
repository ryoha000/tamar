import {
  createSignal,
  createContext,
  useContext,
  ParentComponent,
  Resource,
  Setter,
  Accessor,
} from "solid-js";
import useSearch from "./search";
import { Work } from "./types";

const StoreContext = createContext<Store>();

export interface Store {
  works: Resource<Work[]>;
  setSearchText: Setter<string>;
  workPageMap: Map<string, number>; // key: workId, value: page
  sortKind: Accessor<string>;
  setSortKind: (s: string) => void;
  isSortDesc: Accessor<boolean>;
  setIsSortDesc: (b: boolean) => void;
}

export const StoreProvider: ParentComponent = (props) => {
  const workPageMap = new Map(); // reaactive じゃなくていいためただの Map
  const { works, setText, sortKind, setSortKind, isSortDesc, setIsSortDesc } =
    useSearch();

  const store = {
    works,
    setSearchText: setText,
    workPageMap,
    sortKind,
    setSortKind,
    isSortDesc,
    setIsSortDesc,
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
