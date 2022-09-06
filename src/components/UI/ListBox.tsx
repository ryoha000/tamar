import {
  Component,
  createEffect,
  createSignal,
  For,
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
  isFixed?: boolean;
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
      // TODO: 下にはみ出ることがある
      topLeft = `top: ${_rect.y + _rect.height}px; left: ${_rect.x}px`;
    }
    return widthStyle() + topLeft;
  };

  createEffect(() => {
    if (!target || !props.isOpen) {
      return;
    }
    const _rect = target.getBoundingClientRect();
    setRect(_rect);
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
            class={`${
              props.isFixed ? "fixed" : "absolute"
            } flex flex-col items-center justify-center z-list-box bg-white rounded shadow max-h-40 overflow-y-auto`}
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
