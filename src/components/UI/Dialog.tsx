import { createEffect, onMount, ParentComponent, Show } from "solid-js";
import { Portal } from "solid-js/web";
import EntireOverlayCloser from "./EntireOverlayCloser";

interface Props {
  isOpen: boolean;
  close: () => void;
}

const Dialog: ParentComponent<Props> = (props) => {
  return (
    <Show when={props.isOpen}>
      <Portal>
        <DialogContent isOpen={props.isOpen} close={props.close}>
          {props.children}
        </DialogContent>
      </Portal>
    </Show>
  );
};

const DialogContent: ParentComponent<Props> = (props) => {
  let ele: HTMLDivElement | undefined = undefined;
  onMount(() => {
    ele?.focus();
  });

  return (
    <div
      class="absolute top-0 bg-black bg-opacity-20 left-0 w-full h-full z-dialog"
      tabIndex={-1}
      onkeydown={(e) => {
        if (e.key === "Escape") {
          props.close();
        }
      }}
      onclick={props.close}
      ref={ele}
    >
      <div class="flex items-center justify-center px-12 py-8 h-full">
        <div
          onclick={(e) => e.stopPropagation()}
          class="bg-white rounded px-12 py-8 shadow-md max-h-full overflow-y-scroll min-w-3/5"
        >
          {props.children}
        </div>
      </div>
    </div>
  );
};

export default Dialog;
