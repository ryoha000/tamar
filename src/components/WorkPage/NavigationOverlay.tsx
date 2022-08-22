import { AiOutlineLeft, AiOutlineRight } from "solid-icons/ai";
import { Component, ParentComponent } from "solid-js";
import useHide from "./use/hide";

interface Props {
  navigate: () => void;
}

type BaseProps = {
  class: string;
} & Props;

const NavigationOverlay: ParentComponent<BaseProps> = (props) => {
  const { actionEnd, actionStart, hidden } = useHide();
  return (
    <div
      onclick={props.navigate}
      tabIndex={-1}
      class={`flex items-center h-full absolute top-0 z-work-navigation-overlay p-8 cursor-pointer transition-all ${props.class}`}
      classList={{ "opacity-0": hidden(), "opacity-100": !hidden() }}
      onMouseEnter={actionStart}
      onMouseLeave={actionEnd}
    >
      {props.children}
    </div>
  );
};

export const NextOverlay: Component<Props> = (props) => {
  return (
    <NavigationOverlay class="right-0 w-1/4" navigate={props.navigate}>
      <AiOutlineRight class="ml-auto opacity-50" size="1.5rem" />
    </NavigationOverlay>
  );
};

export const PrevOverlay: Component<Props> = (props) => {
  return (
    <NavigationOverlay class="left-0 w-1/4" navigate={props.navigate}>
      <AiOutlineLeft class="mr-auto opacity-50" size="1.5rem" />
    </NavigationOverlay>
  );
};
