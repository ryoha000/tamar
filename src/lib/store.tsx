import {
  createContext,
  useContext,
  ParentComponent,
  Setter,
  Accessor,
} from "solid-js";
import { SearchWorkRequest } from "./commands";
import useOption from "./option";

const StoreContext = createContext<Store>();

export interface Store {
  setSearchText: Setter<string>;
  workPageMap: Map<string, number>; // key: workId, value: page
  sortKind: Accessor<string>;
  setSortKind: (s: string) => void;
  isSortDesc: Accessor<boolean>;
  setIsSortDesc: (b: boolean) => void;
  sortCol: () => "updated_at" | "title";
  searchRequest: () => SearchWorkRequest;
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
