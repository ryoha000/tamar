import { AiOutlineCloseCircle } from "solid-icons/ai";
import { Component, createSignal, onMount, Show } from "solid-js";
import type { Tag as TagI } from "../../lib/types";

interface Props {
  tag: TagI;
  onclick?: () => void;
  close?: () => void;
}
const randamRGB = () => {
  return Math.floor(Math.random() * 256);
};
const Tag: Component<Props> = (props) => {
  const [bgColor, setBgColor] = createSignal("");
  onMount(() => {
    setBgColor(
      `background-color: rgba(${randamRGB()}, ${randamRGB()}, ${randamRGB()}, 0.5);`
    );
  });
  return (
    <div
      class={`whitespace-nowrap px-4 py-1 text-sm rounded-full flex items-center gap-2 ${
        props.onclick ? "cursor-pointer" : ""
      }`}
      style={bgColor()}
      onclick={props.onclick}
    >
      {props.tag.name}
      <Show when={props.close}>
        <button class="flex items-center" onclick={props.close}>
          <AiOutlineCloseCircle />
        </button>
      </Show>
    </div>
  );
};

export default Tag;
