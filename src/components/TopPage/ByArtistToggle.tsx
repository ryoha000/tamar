import { Component, createSignal } from "solid-js";
import { IoColorPalette } from "solid-icons/io";
import ToggleIconButton from "../UI/ToggleIconButton";

const ByArtistToggle: Component = () => {
  const [isFilter, setIsFilter] = createSignal(true);

  return (
    <ToggleIconButton
      state={isFilter()}
      icon={IoColorPalette}
      onclick={() => setIsFilter((prev) => !prev)}
    />
  );
};

export default ByArtistToggle;
