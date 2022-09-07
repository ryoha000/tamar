import { FaSolidEllipsis } from "solid-icons/fa";
import { Component, createSignal, Show } from "solid-js";
import { useStore } from "../../lib/store";
import { SortKind } from "../../lib/types";
import HeaderNextPrev from "../UI/HeaderNextPrev";
import ByArtistToggle from "./ByArtistToggle";
import MenuDialog from "./MenuDialog";
import SearchInput from "./SearchInput";
import SortSelect from "./SortSelect";

const Header: Component = () => {
  const store = useStore();
  const [isOpenMenuDialog, setIsOpenMenuDialog] = createSignal(false);

  const setSortKind = (v: SortKind) => {
    if (!store) {
      return;
    }
    store.setOffset(0);
    store.setSortKind(v);
  };

  const toggleSortDesc = () => {
    if (!store) {
      return;
    }
    store.setOffset(0);
    store.setIsSortDesc((prev) => !prev);
  };

  const setIsFilterArtist = () => {
    if (!store) {
      return;
    }
    store.setOffset(0);
    store.setIsFilterArtist((prev) => !prev);
  };

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
            select={setSortKind}
            isDesc={store!.isSortDesc()}
            toggleDesc={toggleSortDesc}
          />
          <ByArtistToggle
            isFilter={store!.isFilterArtist()}
            toggle={setIsFilterArtist}
          />
          <button class="ml-auto" onclick={() => setIsOpenMenuDialog(true)}>
            <FaSolidEllipsis size="1.2rem" />
          </button>
        </div>
      </div>
      <MenuDialog
        isOpen={isOpenMenuDialog()}
        close={() => setIsOpenMenuDialog(false)}
        refetch={store!.refetch}
      />
    </Show>
  );
};

export default Header;
