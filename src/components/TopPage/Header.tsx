import { FaSolidEllipsis } from "solid-icons/fa";
import { Component, createSignal, Show } from "solid-js";
import { useStore } from "../../lib/store";
import HeaderNextPrev from "../UI/HeaderNextPrev";
import ByArtistToggle from "./ByArtistToggle";
import FileImportButton from "./FileImportButton";
import MenuDialog from "./MenuDialog";
import SearchInput from "./SearchInput";
import SortSelect from "./SortSelect";

const Header: Component = () => {
  const store = useStore();
  const [isOpenMenuDialog, setIsOpenMenuDialog] = createSignal(false);

  return (
    <Show when={store}>
      <div
        class="fixed top-0 left-0 w-full z-header h-header bg-white"
        data-fixed
      >
        <div class="grid grid-cols-top-header items-center w-full gap-2 px-4 py-2">
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
          <button class="ml-auto" onclick={() => setIsOpenMenuDialog(true)}>
            <FaSolidEllipsis size="1.2rem" />
          </button>
        </div>
      </div>
      <MenuDialog
        isOpen={isOpenMenuDialog()}
        close={() => setIsOpenMenuDialog(false)}
      />
    </Show>
  );
};

export default Header;
