import { Component, createSignal, Setter, Show } from "solid-js";
import { FaSolidArrowDownWideShort } from "solid-icons/fa";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";
import DropDownMenu from "../UI/DropDownMenu";

export const SORT_KIND = ["追加日時", "作品名", "閲覧日時"] as const;
export const INITIAL_SELECT_OPTION = SORT_KIND[0];
export type SortKind = typeof SORT_KIND[number];

interface Props {
  selected: SortKind;
  select: Setter<SortKind>;
  isDesc: boolean;
  toggleDesc: () => void;
}

const SortSelect: Component<Props> = (props) => {
  return (
    <div class="flex items-center gap-1">
      <DropDownMenu
        options={[...SORT_KIND]}
        onChange={(option) => props.select(option as SortKind)}
        selectedOption={props.selected}
      />
      <TooltipWrapper label="降順で表示">
        <ToggleIconButton
          onclick={() => props.toggleDesc()}
          icon={FaSolidArrowDownWideShort}
          state={props.isDesc}
        />
      </TooltipWrapper>
    </div>
  );
};

export default SortSelect;
