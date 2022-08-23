import { Component, createSignal } from "solid-js";
import { IoColorPalette } from "solid-icons/io";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";

interface Props {
  isFilter: boolean;
  toggle: () => void;
}

const ByArtistToggle: Component<Props> = (props) => {
  return (
    <TooltipWrapper label="作者別に表示">
      <ToggleIconButton
        state={props.isFilter}
        icon={IoColorPalette}
        onclick={props.toggle}
      />
    </TooltipWrapper>
  );
};

export default ByArtistToggle;
