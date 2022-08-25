import { Component, onMount, Show } from "solid-js";

interface Props {
  onIntersect: () => void;
  isLoading: boolean;
  isActiveObserver: boolean;
}

const ScrollObserber: Component<Props> = (props) => {
  let marker: HTMLDivElement | undefined = undefined;
  const observer = new IntersectionObserver(
    (entries) => {
      if (entries.length === 0) {
        return;
      }
      if (!props.isLoading && entries[0].intersectionRatio >= 1) {
        console.log(entries[0]);
        props.onIntersect();
      }
    },
    { threshold: 1 }
  );

  onMount(() => {
    if (marker) {
      observer.observe(marker);
    }
  });
  return (
    <Show when={props.isActiveObserver}>
      <div ref={marker} class="w-full h-2 bg-red-700">
        <Show when={props.isLoading}>
          <div>now loading</div>
        </Show>
      </div>
    </Show>
  );
};

export default ScrollObserber;
