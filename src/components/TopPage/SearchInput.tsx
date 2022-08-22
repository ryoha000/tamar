import type { Component } from "solid-js";
import { AiOutlineSearch } from "solid-icons/ai";
import Tag from "../UI/Tag";
import { useStore } from "../../lib/store";

const SearchInput: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  const { setSearchText } = store;
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
        // @ts-ignore
        oninput={(e) => setSearchText(e.target.value)}
      ></input>
    </div>
  );
};

export default SearchInput;
