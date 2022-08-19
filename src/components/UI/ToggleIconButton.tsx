import { Component } from "solid-js";
import { IconProps, IconTypes } from "solid-icons";

interface Props {
  icon: IconTypes;
  state: boolean;
}

const ToggleIconButton: Component<Props & IconProps> = (props) => {
  return (
    <>
      {props.icon({
        ...props,
        class: `cursor-pointer hover:bg-secondary rounded transition-all p-1 ${
          props.class ?? ""
        }`,
        classList: {
          "text-primary": props.state,
          "opacity-50": !props.state,
          ...props.classList,
        },
        size: "1.5rem",
      })}
    </>
  );
};

export default ToggleIconButton;
