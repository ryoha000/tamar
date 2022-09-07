import { AiOutlineArrowLeft, AiOutlineArrowRight } from "solid-icons/ai";
import { Component } from "solid-js";
import { useStore } from "../../lib/store";

const HeaderNextPrev: Component = () => {
  const store = useStore();
  const forward = () => {
    if (store) {
      store.setOffset(0);
    }
    history.forward();
  };
  const back = () => {
    if (store) {
      store.setOffset(0);
    }
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
