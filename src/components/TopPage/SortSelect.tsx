import { Component, createSignal, Show } from "solid-js";
import { FaSolidArrowDownWideShort } from "solid-icons/fa";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";
import DropDownMenu from "../UI/DropDownMenu";
import { useStore } from "../../lib/store";

export const SORT_KIND = ["追加日時", "作品名", "閲覧日時"] as const;
export const INITIAL_SELECT_OPTION = SORT_KIND[0];
export type SortKind = typeof SORT_KIND[number];

const SortSelect: Component = () => {
  const store = useStore();

  return (
    <Show when={store}>
      <div class="flex items-center gap-1">
        <DropDownMenu
          options={[...SORT_KIND]}
          onChange={(option) => store!.setSortKind(option)}
          selectedOption={store!.sortKind()}
        />
        <TooltipWrapper label="降順で表示">
          <ToggleIconButton
            onclick={() => store!.setIsSortDesc(!store?.isSortDesc())}
            icon={FaSolidArrowDownWideShort}
            state={store!.isSortDesc()}
          />
        </TooltipWrapper>
      </div>
    </Show>
  );
};

export default SortSelect;
