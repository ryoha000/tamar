// 参考: https://www.webdesignleaves.com/pr/plugins/css-grid-masonry.html
import { Accessor, Component, createSignal, For, JSX, onMount } from "solid-js";
import MasonryItem from "./MasonryItem";

interface Props<T> {
  each: T[];
  children: (item: T, index: Accessor<number>) => JSX.Element;
}

function MasonryWrapper<T>(props: Props<T>) {
  let container: HTMLDivElement | undefined = undefined;

  const [rowHeight, setRowHeight] = createSignal(0);
  const [rowGap, setRowGap] = createSignal(0);
  onMount(() => {
    if (!container) {
      console.error("MasonryWrapper.container is undefined");
      return;
    }

    const rh = window
      .getComputedStyle(container)
      .getPropertyValue("grid-auto-rows")
      .replace("px", "");
    setRowHeight(+rh);

    const rg = window
      .getComputedStyle(container)
      .getPropertyValue("grid-row-gap")
      .replace("px", "");
    setRowGap(+rg);
  });
  return (
    <div ref={container} class="grid gap-4 grid-cols-masonry-lg auto-rows-0">
      <For each={props.each}>
        {(item, i) => (
          <MasonryItem rowGap={rowGap()} rowHeight={rowHeight()}>
            {props.children(item, i)}
          </MasonryItem>
        )}
      </For>
    </div>
  );
}

export default MasonryWrapper;
