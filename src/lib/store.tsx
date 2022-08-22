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
}

export const StoreProvider: ParentComponent = (props) => {
  const { works, setText } = useSearch();

  const store = {
    works,
    setSearchText: setText,
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
