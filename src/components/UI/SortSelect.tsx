import { Component, createSignal } from "solid-js";
import { FaSolidArrowUpLong, FaSolidArrowDownLong } from "solid-icons/fa";
import ListBox from "./ListBox";

const INITIAL_SELECT_OPTION = "追加日時";
const SortSelect: Component = () => {
  const [selectedOption, setSelectedOption] = createSignal(
    INITIAL_SELECT_OPTION
  );
  const [isOpenSortOptionList, setIsOpenSortOptionList] = createSignal(false);
  const up = () => {
    console.log("up");
  };
  const down = () => {
    console.log("down");
  };

  return (
    <div class="flex items-center gap-1">
      <ListBox
        options={["追加日時", "作品名", "閲覧日時"]}
        onChange={(option) => setSelectedOption(option)}
        close={() => setIsOpenSortOptionList(false)}
        isOpen={isOpenSortOptionList()}
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
          onclick={() => setIsOpenSortOptionList(true)}
          class="w-full px-3 py-1 cursor-pointer"
        >
          {selectedOption()}
        </div>
      </ListBox>
      <div class="flex items-center">
        <FaSolidArrowUpLong class="cursor-pointer" onclick={up} />
        <FaSolidArrowDownLong class="cursor-pointer -m-1" onclick={down} />
      </div>
    </div>
  );
};

export default SortSelect;
