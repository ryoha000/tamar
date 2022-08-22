import { Link } from "@solidjs/router";
import { AiOutlineHome } from "solid-icons/ai";
import { Component, createSignal, onMount } from "solid-js";
import HeaderNextPrev from "../UI/HeaderNextPrev";

const HIDDEN_MS = 1000;
const Header: Component = () => {
  const [hidden, setHidden] = createSignal(false);
  const [timer, setTimer] = createSignal(0);

  onMount(() => {
    setHiddenTimer();
  });
  const setHiddenTimer = () => {
    setTimer(setTimeout(() => setHidden(true), HIDDEN_MS));
  };
  const actionStart = () => {
    clearTimeout(timer());
    setHidden(false);
    console.log("start");
  };
  const actionEnd = () => {
    setHiddenTimer();
    console.log("end");
  };
  return (
    <div
      class="bg-opacity-50 bg-neutral-50 fixed z-header w-full flex items-center gap-2 px-4 py-2 transition-all duration-300 h-header"
      tabIndex={-1}
      classList={{ "opacity-0": hidden(), "opacity-100": !hidden() }}
      onMouseEnter={actionStart}
      onMouseLeave={actionEnd}
    >
      <Link href="/">
        <AiOutlineHome size="1.2rem" />
      </Link>
      <HeaderNextPrev />
      <div>title</div>
    </div>
  );
};

export default Header;
