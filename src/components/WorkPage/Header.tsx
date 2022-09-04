import { Link } from "@solidjs/router";
import { AiOutlineHome } from "solid-icons/ai";
import { FaSolidEllipsis } from "solid-icons/fa";
import { IoGridOutline } from "solid-icons/io";
import { Component } from "solid-js";
import HeaderNextPrev from "../UI/HeaderNextPrev";
import useHide from "./use/hide";

interface Props {
  openListDialog: () => void;
  openMenuDialog: () => void;
  workTitle: string;
}

const Header: Component<Props> = (props) => {
  const { actionStart, actionEnd, hidden } = useHide();

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
        <Link href="/">
          <AiOutlineHome size="1.2rem" />
        </Link>
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
