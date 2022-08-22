import { AiOutlineLeft, AiOutlineRight } from "solid-icons/ai";
import { Component, ParentComponent } from "solid-js";

interface Props {
  navigate: () => void;
}

type BaseProps = {
  class: string;
} & Props;

const NavigationOverlay: ParentComponent<BaseProps> = (props) => {
  return (
    <div
      onclick={props.navigate}
      tabIndex={-1}
      class={`flex items-center h-full absolute top-0 z-work-navigation-overlay p-8 cursor-pointer ${props.class}`}
    >
      {props.children}
    </div>
  );
};

export const NextOverlay: Component<Props> = (props) => {
  return (
    <NavigationOverlay class="right-0 w-2/3" navigate={props.navigate}>
      <AiOutlineRight class="ml-auto" size="1.5rem" />
    </NavigationOverlay>
  );
};

export const PrevOverlay: Component<Props> = (props) => {
  return (
    <NavigationOverlay class="left-0 w-1/3" navigate={props.navigate}>
      <AiOutlineLeft class="mr-auto" size="1.5rem" />
    </NavigationOverlay>
  );
};
