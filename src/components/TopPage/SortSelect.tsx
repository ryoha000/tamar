import { Component, Setter } from "solid-js";
import { FaSolidArrowDownWideShort } from "solid-icons/fa";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";
import DropDownMenu from "../UI/DropDownMenu";
import { SortKind, SORT_KIND } from "../../lib/types";

interface Props {
  selected: SortKind;
  select: (v: SortKind) => void;
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
        isFixed={true}
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
