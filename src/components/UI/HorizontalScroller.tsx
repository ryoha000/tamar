import { IconTypes } from "solid-icons";
import { FaSolidAngleLeft, FaSolidAngleRight } from "solid-icons/fa";
import {
  Component,
  createSignal,
  onMount,
  ParentComponent,
  Show,
} from "solid-js";

type ControllerProps = {
  class: string;
  gradientClass: string;
  icon: IconTypes;
} & Pick<Props, "iconSize" | "isGradientFader">;

type ClickProps = {
  onclick: () => void;
  canScroll: boolean;
} & Pick<Props, "iconSize" | "isGradientFader">;

interface Props {
  isGradientFader: boolean;
  scrollStep: number;
  iconSize: "sm" | "md";
}

const HorizontalScroller: ParentComponent<Props> = (props) => {
  let container: HTMLDivElement | undefined = undefined;

  const [canLeft, setCanLeft] = createSignal(false);
  const [canRight, setCanRight] = createSignal(false);

  onMount(() => {
    setCanScroll();

    if (!container) {
      return;
    }
    const observer = new ResizeObserver(() => {
      setCanScroll();
    });
    observer.observe(container);
  });

  const setCanScroll = () => {
    if (!container) {
      return;
    }
    const rect = container.getBoundingClientRect();
    const scrollWidth = container.scrollWidth;

    // 横幅がおさまってるとき
    if (rect.width >= scrollWidth) {
      setCanLeft(false);
      setCanRight(false);
      return;
    }

    const scrollLeft = container.scrollLeft;
    setCanLeft(scrollLeft > 0);

    const scrollRight = scrollWidth - rect.width - scrollLeft;
    setCanRight(scrollRight > 0);
  };

  const left = () => {
    if (container) {
      container.scrollBy(-1 * props.scrollStep, 0);
    }
  };
  const right = () => {
    if (container) {
      container.scrollBy(props.scrollStep, 0);
    }
  };
  return (
    <div class="relative w-full">
      <div
        class="overflow-x-auto hidden-scrollbar group scroll-smooth"
        ref={container}
        onscroll={setCanScroll}
      >
        <HorizontalScrollerLeft
          onclick={left}
          canScroll={canLeft()}
          isGradientFader={props.isGradientFader}
          iconSize={props.iconSize}
        />
        <HorizontalScrollerRight
          onclick={right}
          canScroll={canRight()}
          isGradientFader={props.isGradientFader}
          iconSize={props.iconSize}
        />
        {props.children}
      </div>
    </div>
  );
};

const HorizontalScrollerControl: Component<ControllerProps & ClickProps> = (
  props
) => {
  const iconSize = () => (props.iconSize === "md" ? "1.2rem" : "1.0rem");
  return (
    <button
      class={`absolute top-0 h-full z-artist-navigation-overlay cursor-pointer ${props.class}`}
      onclick={props.onclick}
    >
      <div class="relative flex items-center h-full p-2">
        <div
          class={`${props.iconSize === "md" ? "p-2" : "p-1"} ${
            props.canScroll ? "" : "hidden"
          } rounded-full bg-white-opacity-70 transition-all duration-500 opacity-0 group-hover:opacity-100`}
        >
          {props.icon({
            size: iconSize(),
            class: "opacity-70",
          })}
        </div>
        <Show when={props.isGradientFader}>
          <div
            class={`absolute top-0 h-full w-1/2 ${props.gradientClass} ${props.class}`}
          />
        </Show>
      </div>
    </button>
  );
};

const HorizontalScrollerLeft: Component<ClickProps> = (props) => {
  return (
    <HorizontalScrollerControl
      isGradientFader={props.isGradientFader}
      iconSize={props.iconSize}
      canScroll={props.canScroll}
      onclick={props.onclick}
      class="left-0"
      gradientClass="bg-gradient-to-r from-white"
      icon={FaSolidAngleLeft}
    />
  );
};

const HorizontalScrollerRight: Component<ClickProps> = (props) => {
  return (
    <HorizontalScrollerControl
      isGradientFader={props.isGradientFader}
      iconSize={props.iconSize}
      canScroll={props.canScroll}
      onclick={props.onclick}
      class="right-0"
      gradientClass="bg-gradient-to-l from-white"
      icon={FaSolidAngleRight}
    />
  );
};

export default HorizontalScroller;
