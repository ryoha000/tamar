import { AiOutlineArrowLeft, AiOutlineArrowRight } from "solid-icons/ai";
import { Component } from "solid-js";

const HeaderNextPrev: Component = () => {
  const forward = () => {
    history.forward();
  };
  const back = () => {
    history.back();
  };

  return (
    <div class="flex items-center gap-2">
      <AiOutlineArrowLeft size="1.2rem" onclick={back} class="cursor-pointer" />
      <AiOutlineArrowRight
        size="1.2rem"
        onclick={forward}
        class="cursor-pointer"
      />
    </div>
  );
};

export default HeaderNextPrev;
