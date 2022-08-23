import { IconTypes } from "solid-icons";
import { FaSolidAngleLeft, FaSolidAngleRight } from "solid-icons/fa";
import { Component, createSignal, onMount, ParentComponent } from "solid-js";

interface Props {
  class: string;
  gradientClass: string;
  icon: IconTypes;
}

interface ClickProps {
  onclick: () => void;
  canScroll: boolean;
}

const ArtistScroller: ParentComponent = (props) => {
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
      container.scrollBy(-300, 0);
    }
  };
  const right = () => {
    if (container) {
      container.scrollBy(300, 0);
    }
  };
  return (
    <div class="relative">
      <div
        class="flex overflow-x-auto gap-4 p-4 hidden-scrollbar group scroll-smooth"
        ref={container}
        onscroll={setCanScroll}
      >
        <ArtistScrollerLeft onclick={left} canScroll={canLeft()} />
        <ArtistScrollerRight onclick={right} canScroll={canRight()} />
        {props.children}
      </div>
    </div>
  );
};

const ArtistScrollerControl: Component<Props & ClickProps> = (props) => {
  return (
    <button
      class={`absolute top-0 h-full z-artist-navigation-overlay cursor-pointer ${props.class}`}
      onclick={props.onclick}
    >
      <div class="relative flex items-center h-full p-2">
        <div
          classList={{ hidden: !props.canScroll }}
          class="p-2 rounded-full bg-white-opacity-70 transition-all duration-500 opacity-0 group-hover:opacity-100"
        >
          {props.icon({
            size: "1.2rem",
            class: "opacity-70",
          })}
        </div>
        <div
          class={`absolute top-0 h-full w-1/2 ${props.gradientClass} ${props.class}`}
        />
      </div>
    </button>
  );
};

const ArtistScrollerLeft: Component<ClickProps> = (props) => {
  return (
    <ArtistScrollerControl
      canScroll={props.canScroll}
      onclick={props.onclick}
      class="left-0"
      gradientClass="bg-gradient-to-r from-white"
      icon={FaSolidAngleLeft}
    />
  );
};

const ArtistScrollerRight: Component<ClickProps> = (props) => {
  return (
    <ArtistScrollerControl
      canScroll={props.canScroll}
      onclick={props.onclick}
      class="right-0"
      gradientClass="bg-gradient-to-l from-white"
      icon={FaSolidAngleRight}
    />
  );
};

export default ArtistScroller;
