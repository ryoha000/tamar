import type { Component } from "solid-js";
import { AiOutlineSearch } from "solid-icons/ai";

const SearchInput: Component = () => {
  return (
    <div class="flex items-center border-solid border-text border rounded-full px-3 py-1 gap-2 transition-all focus-within:border-accent">
      <AiOutlineSearch />
      <div>tag</div>
      <input class="w-full focus:outline-none"></input>
    </div>
  );
};

export default SearchInput;
