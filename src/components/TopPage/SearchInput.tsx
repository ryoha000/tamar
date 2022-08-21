import type { Component } from "solid-js";
import { AiOutlineSearch } from "solid-icons/ai";
import Tag from "../UI/Tag";

const SearchInput: Component = () => {
  return (
    <div class="flex items-center border-solid border-text border rounded-full px-3 py-1 gap-2 transition-all focus-within:border-accent flex-1">
      <AiOutlineSearch />
      <Tag // TODO: 検索中のタグ一覧表示
        tag={{ id: "", name: "tag name", createdAt: "", updatedAt: "" }}
        isCloseIcon={true}
        close={() => {}}
      />
      <input class="w-full focus:outline-none"></input>
    </div>
  );
};

export default SearchInput;
