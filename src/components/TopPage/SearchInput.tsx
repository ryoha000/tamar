import { Accessor, Component, For, Setter } from "solid-js";
import { AiOutlineSearch } from "solid-icons/ai";
import Tag from "../UI/Tag";
import useSuggest, { UseSuggestProps } from "./use/suggest";
import { Tag as TagI } from "../../lib/types";
import SearchTagList from "./SearchTagList";

type Props = {
  tags: TagI[];
} & UseSuggestProps;

const SearchInput: Component<Props> = (props) => {
  const { keydown, input, change, options } = useSuggest(props);
  return (
    <div class="flex items-center flex-1 border-solid border-text border rounded-full px-3 py-1 gap-2 transition-all focus-within:border-accent">
      <AiOutlineSearch />
      <SearchTagList tags={props.tags} />
      <input
        class="focus:outline-none flex-1"
        type="text"
        list="search"
        onkeydown={keydown}
        oninput={input}
        onchange={change}
      ></input>
      <datalist id="search">
        <For each={options()}>{(option, i) => <option>{option}</option>}</For>
      </datalist>
    </div>
  );
};

export default SearchInput;
