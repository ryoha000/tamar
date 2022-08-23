import { Component, For, Setter } from "solid-js";
import { AiOutlineSearch } from "solid-icons/ai";
import Tag from "../UI/Tag";
import useSuggest from "./use/suggest";

interface Props {
  setText: Setter<string>;
}

const SearchInput: Component<Props> = (props) => {
  const options = ["aaaaa", "bbbbb", "vvvvv", "aacccc"];
  const { keydown, input } = useSuggest(props);
  return (
    <div class="flex items-center border-solid border-text border rounded-full px-3 py-1 gap-2 transition-all focus-within:border-accent flex-1">
      <AiOutlineSearch />
      <Tag // TODO: 検索中のタグ一覧表示
        tag={{ id: "", name: "tag name", updatedAt: "" }}
        isCloseIcon={true}
        close={() => {}}
      />
      <input
        class="w-full focus:outline-none"
        type="text"
        list="search"
        onkeydown={keydown}
        oninput={input}
        // oninput={(e) => props.setText(e.target.value)}
      ></input>
      <datalist id="search">
        <For each={options}>{(option, i) => <option>{option}</option>}</For>
      </datalist>
    </div>
  );
};

export default SearchInput;
