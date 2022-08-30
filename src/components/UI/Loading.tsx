import { Component } from "solid-js";

const Loading: Component = () => {
  return (
    <div class="flex justify-center p-4">
      <div class="animate-spin h-10 w-10 border-2 border-text-opacity-50 rounded-full border-t-transparent"></div>
    </div>)
}

export default Loading
