import { Component, createSignal } from "solid-js";
import { FaSolidArrowDownWideShort } from "solid-icons/fa";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";
import DropDownMenu from "../UI/DropDownMenu";

const INITIAL_SELECT_OPTION = "追加日時";
const SortSelect: Component = () => {
  const [selectedOption, setSelectedOption] = createSignal(
    INITIAL_SELECT_OPTION
  );

  const [isDesc, setIsDesc] = createSignal(true);

  return (
    <div class="flex items-center gap-1">
      <DropDownMenu
        options={["追加日時", "作品名", "閲覧日時"]}
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
