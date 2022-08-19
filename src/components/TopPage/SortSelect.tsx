import { Component, createSignal, ParentComponent } from "solid-js";
import {
  FaSolidArrowUpLong,
  FaSolidArrowDownLong,
  FaSolidArrowDownWideShort,
} from "solid-icons/fa";
import ListBox from "../UI/ListBox";

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
        width="6rem"
      >
        <div
          onclick={() => setIsOpenOptionList(true)}
          class="w-full px-3 py-1 cursor-pointer hover:bg-background rounded transition-all"
        >
          {selectedOption()}
        </div>
      </ListBox>
      <FaSolidArrowDownWideShort
        onclick={() => setIsDesc((prev) => !prev)}
        class="cursor-pointer hover:bg-secondary rounded transition-all p-1"
        classList={{
          "text-primary": isDesc(),
          "opacity-50": !isDesc(),
        }}
        size="1.5rem"
      />
    </div>
  );
};

export default SortSelect;
