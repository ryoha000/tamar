// 参考: https://www.webdesignleaves.com/pr/plugins/css-grid-masonry.html
import { Component, createSignal, onMount } from "solid-js";

interface Props {
  children: Component<{ rowHeight: number; rowGap: number }>;
}

const MasonryWrapper: Component<Props> = (props) => {
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
    <div>
      <div ref={container} class="grid gap-4 grid-cols-masonry-lg auto-rows-0">
        {props.children({ rowHeight: rowHeight(), rowGap: rowGap() })}
      </div>
    </div>
  );
};

export default MasonryWrapper;
