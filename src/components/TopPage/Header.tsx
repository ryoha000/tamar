import { Component } from "solid-js";
import ByArtistToggle from "./ByArtistToggle";
import FileImportButton from "./FileImportButton";
import SearchInput from "./SearchInput";
import SortSelect from "./SortSelect";

const Header: Component = () => {
  return (
    <div class="flex items-center bg-white px-4 py-2 gap-2">
      <SearchInput />
      <SortSelect />
      <ByArtistToggle />
      <FileImportButton />
    </div>
  );
};

export default Header;