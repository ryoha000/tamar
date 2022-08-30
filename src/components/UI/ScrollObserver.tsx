import { Component, onMount, Show } from "solid-js";
import Loading from "./Loading";

type Props = {
  isActiveObserver: boolean;
} & MarkerProps

interface MarkerProps {
  onIntersect: () => void;
  isLoading: boolean;
}

const ScrollObserber: Component<Props> = (props) => {
  return (
    <Show when={props.isActiveObserver}>
      <ScrollObserberMarker onIntersect={props.onIntersect} isLoading={props.isLoading} />
    </Show>
  );
};

const ScrollObserberMarker: Component<MarkerProps> = (props) => {
  let marker: HTMLDivElement | undefined = undefined;
  const observer = new IntersectionObserver(
    (entries) => {
      if (entries.length === 0) {
        return;
      }
      if (!props.isLoading && entries[0].intersectionRatio >= 1) {
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
    <div ref={marker} class="w-full h-2 bg-red-700">
      <Show when={props.isLoading}>
        <Loading />
      </Show>
    </div>
  );
};

export default ScrollObserber;
