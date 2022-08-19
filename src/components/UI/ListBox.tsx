import {
  Component,
  createEffect,
  createSignal,
  For,
  onMount,
  ParentComponent,
  Show,
} from "solid-js";
import { Portal } from "solid-js/web";
import EntireOverlayCloser from "./EntireOverlayCloser";

interface Props {
  options: string[];
  onChange: (option: string) => void;
  close: () => void;
  isOpen: boolean;
  width: string;
  optionComponent: Component<OptionProps>;
}

export interface OptionProps {
  option: string;
  select: (option: string) => void;
}

const ListBox: ParentComponent<Props> = (props) => {
  let target: HTMLDivElement | undefined = undefined;

  const [rect, setRect] = createSignal<DOMRect | null>(null);
  const widthStyle = () => `width: ${props.width};`;
  const portalStyle = () => {
    let topLeft = "";
    const _rect = rect();
    if (_rect) {
      topLeft = `top: ${_rect.y + _rect.height}px; left: ${_rect.x}px`;
    }
    return widthStyle() + topLeft;
  };

  onMount(() => {
    if (!target) {
      return;
    }
    const _rect = target.getBoundingClientRect();
    if (_rect) {
      setRect(_rect);
    }
  });

  const selectOption = (option: string) => {
    props.onChange(option);
    props.close();
  };

  return (
    <div style={widthStyle()}>
      <div class="w-full flex items-center justify-center" ref={target}>
        {props.children}
      </div>
      <Show when={props.isOpen}>
        <Portal>
          <div
            class="absolute flex flex-col items-center justify-center z-list-box bg-white rounded shadow"
            style={portalStyle()}
            onclick={(e) => e.stopPropagation()}
          >
            <For each={props.options}>
              {(option, i) =>
                props.optionComponent({ option, select: selectOption })
              }
            </For>
          </div>
          <EntireOverlayCloser class="z-list-box-overlay" close={props.close} />
        </Portal>
      </Show>
    </div>
  );
};

export default ListBox;
