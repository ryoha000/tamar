import { useNavigate } from "@solidjs/router";
import { AiOutlineCheck, AiOutlineClose, AiOutlineEdit } from "solid-icons/ai";
import { Component, createSignal, For, Match, Show, Switch } from "solid-js";
import useEdit, { EditProps } from "./use/edit";

type Props = {
  inputClass?: string;
  link?: string; // editable === false のときに input をクリックするとジャンプする先
};

const Editor: Component<Props & EditProps> = (props) => {
  let inputEle: HTMLInputElement | undefined = undefined;

  const {
    startEdit,
    editable,
    input,
    confirmEdit,
    cancelEdit,
    suggests,
    text,
  } = useEdit(props);

  const clickStartOrConfirm = async (e: Event) => {
    e.preventDefault();
    if (editable()) {
      // form経由の時はfocusoutが発生しない？
      setIsFocusInput(false);
      await confirmEdit();
    } else {
      startEdit();
      if (inputEle) {
        inputEle.focus();
      }
    }
  };

  const stop = (e: Event) => {
    e.stopPropagation();
  };

  const randomString = `editor-${Math.random()}`;
  const navigator = useNavigate();
  const clickInput = () => {
    if (clickable()) {
      navigator(props.link!);
    }
  };

  const clickable = () => !editable() && props.link;

  const [isFocusInput, setIsFocusInput] = createSignal(false);
  return (
    <form
      class="flex items-center gap-4 w-full"
      onsubmit={clickStartOrConfirm}
      oninput={stop}
      onclick={stop}
      onkeydown={stop}
    >
      <div class="relative flex-1">
        <input
          list={randomString}
          class={`flex-1 rounded transition-all ${
            clickable() ? "hover:bg-secondary" : ""
          } ${clickable() ? "cursor-pointer" : ""} ${props.inputClass ?? ""}`}
          value={text()}
          oninput={input}
          readOnly={!editable()}
          onclick={clickInput}
          onfocus={() => setIsFocusInput(editable() && true)}
          onfocusout={() => setIsFocusInput(false)}
          ref={inputEle}
        />
        <div
          classList={{
            "scale-0": !isFocusInput(),
            "scale-100": isFocusInput() && editable(),
          }}
          class="absolute bottom-0 left-0 h-0.5 w-full bg-secondary transition-all"
        ></div>
      </div>
      <Show when={!!props.fetchSuggests}>
        <datalist id={randomString}>
          <For each={suggests()}>
            {(suggest, i) => <option>{suggest}</option>}
          </For>
        </datalist>
      </Show>
      <Switch>
        <Match when={editable()}>
          <button
            type="button"
            class="opacity-50 hover:opacity-80 transition-all"
            onclick={clickStartOrConfirm}
          >
            <AiOutlineCheck size="1.2rem" />
          </button>
        </Match>
        <Match when={!editable()}>
          <button
            type="submit"
            class="opacity-50 hover:opacity-80 transition-all"
          >
            <AiOutlineEdit size="1.2rem" />
          </button>
        </Match>
      </Switch>
      <button
        type="button"
        onclick={cancelEdit}
        classList={{ "opacity-100": editable(), "opacity-0": !editable() }}
      >
        <AiOutlineClose
          class="text-error opacity-50 hover:opacity-80 transition-all"
          size="1.2rem"
        />
      </button>
    </form>
  );
};

export default Editor;
