import {
  createSignal,
  createContext,
  useContext,
  ParentComponent,
  Resource,
  Setter,
} from "solid-js";
import useSearch from "./search";
import { Work } from "./types";

const StoreContext = createContext<Store>();

export interface Store {
  works: Resource<Work[]>;
  setSearchText: Setter<string>;
  workPageMap: Map<string, number>; // key: workId, value: page
}

export const StoreProvider: ParentComponent = (props) => {
  const { works, setText } = useSearch();
  const workPageMap = new Map(); // reaactive じゃなくていいためただの Map

  const store = {
    works,
    setSearchText: setText,
    workPageMap,
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
