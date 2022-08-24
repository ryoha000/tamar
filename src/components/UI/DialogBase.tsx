import { onCleanup, onMount, ParentComponent, Show } from "solid-js";
import { Portal } from "solid-js/web";

interface Props {
  isOpen: boolean;
  close: () => void;
  withCurtain: boolean;
  align: "center" | "left";
}

const DialogBase: ParentComponent<Props> = (props) => {
  return (
    <Show when={props.isOpen}>
      <Portal>
        <DialogBaseContent
          isOpen={props.isOpen}
          close={props.close}
          withCurtain={props.withCurtain}
          align={props.align}
        >
          {props.children}
        </DialogBaseContent>
      </Portal>
    </Show>
  );
};

const getScrollbarWidth = () => {
  let element = document.createElement("div");
  element.style.visibility = "hidden";
  element.style.overflow = "scroll";
  document.body.appendChild(element);
  const scrollbarWidth = element.offsetWidth - element.clientWidth;
  document.body.removeChild(element);

  return scrollbarWidth;
};

const scrollbarVisible = () => {
  return window.innerWidth > document.body.clientWidth;
};

const DialogBaseContent: ParentComponent<Props> = (props) => {
  let ele: HTMLDivElement | undefined = undefined;
  onMount(() => {
    ele?.focus();
    if (scrollbarVisible()) {
      document.body.style.paddingRight = `${getScrollbarWidth()}px`;
    }
    window.document.body.style.overflowY = "hidden";
  });
  onCleanup(() => {
    window.document.body.style.overflowY = "auto";
    document.body.style.paddingRight = `0px`;
  });

  return (
    <div
      class={`absolute top-0 ${
        props.withCurtain ? "bg-black bg-opacity-20" : ""
      } left-0 w-full h-full z-dialog`}
      tabIndex={-1}
      onkeydown={(e) => {
        if (e.key === "Escape") {
          props.close();
        }
      }}
      onclick={props.close}
      ref={ele}
    >
      <div
        class={`flex items-center ${
          props.align === "left" ? "justify-end" : "justify-center"
        } px-12 py-8 h-full`}
      >
        <div
          onclick={(e) => e.stopPropagation()}
          class={`bg-white rounded px-12 py-8 shadow-md max-h-full overflow-y-auto ${
            props.align === "center" ? "min-w-3/5" : ""
          }`}
        >
          {props.children}
        </div>
      </div>
    </div>
  );
};

export default DialogBase;
