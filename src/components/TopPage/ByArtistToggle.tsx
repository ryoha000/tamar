import { Component, createSignal } from "solid-js";
import { IoColorPalette } from "solid-icons/io";
import ToggleIconButton from "../UI/ToggleIconButton";
import TooltipWrapper from "../UI/TooltipWrapper";

const ByArtistToggle: Component = () => {
  const [isFilter, setIsFilter] = createSignal(true);

  return (
    <TooltipWrapper label="作者別に表示">
      <ToggleIconButton
        state={isFilter()}
        icon={IoColorPalette}
        onclick={() => setIsFilter((prev) => !prev)}
      />
    </TooltipWrapper>
  );
};

export default ByArtistToggle;
