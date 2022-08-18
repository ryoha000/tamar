import { Component } from "solid-js";

interface Props {
  close: () => void;
  class?: string;
}

const EntireOverlayCloser: Component<Props> = (props) => {
  const click = (e: MouseEvent) => {
    e.stopPropagation();
    props.close();
  };
  return (
    <div
      class={`absolute top-0 left-0 w-full h-full z-entire-overlay ${
        props.class ?? ""
      }`}
      onclick={click}
    ></div>
  );
};

export default EntireOverlayCloser;
