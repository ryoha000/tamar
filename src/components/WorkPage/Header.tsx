import { Link, useNavigate } from "@solidjs/router";
import { AiOutlineHome } from "solid-icons/ai";
import { FaSolidEllipsis } from "solid-icons/fa";
import { IoGridOutline } from "solid-icons/io";
import { Component } from "solid-js";
import { useStore } from "../../lib/store";
import HeaderNextPrev from "../UI/HeaderNextPrev";
import useHide from "./use/hide";

interface Props {
  openListDialog: () => void;
  openMenuDialog: () => void;
  workTitle: string;
}

const Header: Component<Props> = (props) => {
  const { actionStart, actionEnd, hidden } = useHide();

  const store = useStore();
  const navigator = useNavigate();
  const goTopPage = () => {
    if (store) {
      store.setOffset(0);
    }
    navigator("/");
  };

  return (
    <div
      class="bg-opacity-50 bg-neutral-50 fixed z-header w-full transition-all duration-300 h-header"
      tabIndex={-1}
      classList={{ "opacity-0": hidden(), "opacity-100": !hidden() }}
      onMouseEnter={actionStart}
      onMouseLeave={actionEnd}
      data-fixed
    >
      <div class="w-full flex items-center gap-2 px-4 py-2">
        <div onclick={goTopPage} class="cursor-pointer">
          <AiOutlineHome size="1.2rem" />
        </div>
        <HeaderNextPrev />
        <IoGridOutline
          size="1.2rem"
          class="cursor-pointer"
          onclick={props.openListDialog}
        />
        <div class="ml-8">{props.workTitle}</div>
        <button class="ml-auto" onclick={props.openMenuDialog}>
          <FaSolidEllipsis size="1.2rem" />
        </button>
      </div>
    </div>
  );
};

export default Header;
