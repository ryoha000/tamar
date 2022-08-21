import { Component, createSignal } from "solid-js";
import { FaSolidArrowDownWideShort } from "solid-icons/fa";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";
import DropDownMenu from "../UI/DropDownMenu";

export const SORT_KIND = ["追加日時", "作品名", "閲覧日時"] as const;
const INITIAL_SELECT_OPTION = SORT_KIND[0];
export type SortKind = typeof SORT_KIND[number];

const SortSelect: Component = () => {
  const [selectedOption, setSelectedOption] = createSignal<string>(
    INITIAL_SELECT_OPTION
  );

  const [isDesc, setIsDesc] = createSignal(true);

  return (
    <div class="flex items-center gap-1">
      <DropDownMenu
        options={[...SORT_KIND]}
        onChange={(option) => setSelectedOption(option)}
        selectedOption={selectedOption()}
      />
      <TooltipWrapper label="降順で表示">
        <ToggleIconButton
          onclick={() => setIsDesc((prev) => !prev)}
          icon={FaSolidArrowDownWideShort}
          state={isDesc()}
        />
      </TooltipWrapper>
    </div>
  );
};

export default SortSelect;
