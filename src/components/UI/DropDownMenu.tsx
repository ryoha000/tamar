import { Component, createSignal } from "solid-js";
import ListBox from "../UI/ListBox";
import { AiOutlineCaretDown } from "solid-icons/ai";

interface Props {
  options: string[];
  selectedOption: string;
  width?: string;
  onChange: (option: string) => void;
  isFixed?: boolean;
}

const DropDownMenu: Component<Props> = (props) => {
  const [isOpenOptionList, setIsOpenOptionList] = createSignal(false);

  return (
    <ListBox
      options={props.options}
      onChange={props.onChange}
      close={() => setIsOpenOptionList(false)}
      isOpen={isOpenOptionList()}
      optionComponent={({ option, select }) => (
        <div
          onclick={() => select(option)}
          class="w-full px-3 py-1 cursor-pointer hover:bg-secondary rounded transition-all"
          classList={{
            "bg-background": option === props.selectedOption,
          }}
        >
          {option}
        </div>
      )}
      width={props.width ?? "7rem"}
      isFixed={props.isFixed}
    >
      <div
        onclick={() => setIsOpenOptionList(true)}
        class="w-full px-3 py-1 cursor-pointer hover:bg-background rounded transition-all flex items-center gap-2"
      >
        {props.selectedOption}
        <AiOutlineCaretDown
          classList={{ "rotate-180": isOpenOptionList() }}
          class="transition-all"
        />
      </div>
    </ListBox>
  );
};

export default DropDownMenu;
