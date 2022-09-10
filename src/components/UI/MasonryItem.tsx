import { onMount, ParentComponent } from "solid-js";

interface Props {
  rowHeight: number;
  rowGap: number;
}

const MasonryItem: ParentComponent<Props> = (props) => {
  let container: HTMLDivElement | undefined = undefined;
  let content: HTMLDivElement | undefined = undefined;

  const resizeObserver = new ResizeObserver((entries) => {
    if (!container || !content) {
      console.error(
        "MasonryItem.(container|content) is undefined, when resizing."
      );
      return;
    }

    const rowSpan = Math.ceil(
      (content.getBoundingClientRect().height + props.rowGap) /
        (props.rowHeight + props.rowGap)
    );
    container.style.gridRowEnd = `span ${rowSpan}`;
  });

  onMount(() => {
    if (!container || !content) {
      console.error("MasonryItem.(container|content) is undefined");
      return;
    }

    const rowSpan = Math.ceil(
      (content.getBoundingClientRect().height + props.rowGap) /
        (props.rowHeight + props.rowGap)
    );
    container.style.gridRowEnd = `span ${rowSpan}`;

    resizeObserver.observe(content);
  });
  return (
    <div ref={container}>
      <div ref={content}>{props.children}</div>
    </div>
  );
};

export default MasonryItem;
