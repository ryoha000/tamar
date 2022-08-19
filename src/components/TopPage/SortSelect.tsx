import { Component, createSignal } from "solid-js";
import { FaSolidArrowDownWideShort } from "solid-icons/fa";
import ListBox from "../UI/ListBox";
import ToggleIconButton from "../UI/ToggleIconButton";
import { AiOutlineCaretDown } from "solid-icons/ai";

const INITIAL_SELECT_OPTION = "追加日時";
const SortSelect: Component = () => {
  const [selectedOption, setSelectedOption] = createSignal(
    INITIAL_SELECT_OPTION
  );
  const [isOpenOptionList, setIsOpenOptionList] = createSignal(false);

  const [isDesc, setIsDesc] = createSignal(true);

  return (
    <div class="flex items-center gap-1">
      <ListBox
        options={["追加日時", "作品名", "閲覧日時"]}
        onChange={(option) => setSelectedOption(option)}
        close={() => setIsOpenOptionList(false)}
        isOpen={isOpenOptionList()}
        optionComponent={({ option, select }) => (
          <div
            onclick={() => select(option)}
            class="w-full px-3 py-1 cursor-pointer hover:bg-secondary rounded transition-all"
            classList={{
              "bg-background": option === selectedOption(),
            }}
          >
            {option}
          </div>
        )}
        width="7rem"
      >
        <div
          onclick={() => setIsOpenOptionList(true)}
          class="w-full px-3 py-1 cursor-pointer hover:bg-background rounded transition-all flex items-center gap-2"
        >
          {selectedOption()}
          <AiOutlineCaretDown
            classList={{ "rotate-180": isOpenOptionList() }}
            class="transition-all"
          />
        </div>
      </ListBox>
      <ToggleIconButton
        onclick={() => setIsDesc((prev) => !prev)}
        icon={FaSolidArrowDownWideShort}
        state={isDesc()}
      />
    </div>
  );
};

export default SortSelect;
