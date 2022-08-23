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
      <div class="flex items-center bg-white px-4 py-2 gap-2 fixed top-0 left-0 w-full z-header h-header">
        <HeaderNextPrev />
        <SearchInput setText={store!.setSearchText} />
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
    </Show>
  );
};

export default Header;
