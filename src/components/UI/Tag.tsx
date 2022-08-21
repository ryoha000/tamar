import { AiOutlineCloseCircle } from "solid-icons/ai";
import { Component, createSignal, onMount, Show } from "solid-js";
import type { Tag as TagI } from "../../lib/types";

interface Props {
  tag: TagI;
  isCloseIcon?: boolean;
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
      class="whitespace-nowrap px-4 py-1 text-sm rounded-full flex items-center gap-2"
      style={bgColor()}
    >
      {props.tag.name}
      <Show when={props.isCloseIcon && props.close}>
        <button class="flex items-center">
          <AiOutlineCloseCircle />
        </button>
      </Show>
    </div>
  );
};

export default Tag;
