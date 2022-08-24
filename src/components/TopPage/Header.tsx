import { Component, Show } from "solid-js";
import useOption from "../../lib/option";
import { useStore } from "../../lib/store";
import HeaderNextPrev from "../UI/HeaderNextPrev";
import ByArtistToggle from "./ByArtistToggle";
import FileImportButton from "./FileImportButton";
import SearchInput from "./SearchInput";
import SortSelect from "./SortSelect";

const Header: Component = () => {
  const store = useStore();

  return (
    <Show when={store}>
      <div class="fixed top-0 left-0 w-full z-header h-header px-4 py-2 bg-white">
        <div class="grid grid-cols-top-header items-center w-full gap-2">
          {/* <div class="flex items-center w-full gap-2"> */}
          <HeaderNextPrev />
          <SearchInput
            setText={store!.setSearchText}
            setTags={store!.setSearchTags}
            tags={store!.searchTags()}
          />
          <SortSelect
            selected={store!.sortKind()}
            select={store!.setSortKind}
            isDesc={store!.isSortDesc()}
            toggleDesc={() => store!.setIsSortDesc((prev) => !prev)}
          />
          <ByArtistToggle
            isFilter={store!.isFilterArtist()}
            toggle={() => store!.setIsFilterArtist((prev) => !prev)}
          />
          <FileImportButton />
        </div>
      </div>
    </Show>
  );
};

export default Header;
