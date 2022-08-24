import { useNavigate } from "@solidjs/router";
import { AiOutlineCheck, AiOutlineClose, AiOutlineEdit } from "solid-icons/ai";
import { Component, createSignal, For, Match, Show, Switch } from "solid-js";
import useEdit, { EditProps } from "./use/edit";

type Props = {
  inputClass?: string;
  link?: string; // editable === false のときに input をクリックするとジャンプする先
};

const Editor: Component<Props & EditProps> = (props) => {
  const {
    startEdit,
    editable,
    input,
    confirmEdit,
    cancelEdit,
    suggests,
    text,
  } = useEdit(props);

  const clickStartOrConfirm = async () => {
    if (editable()) {
      await confirmEdit();
    } else {
      startEdit();
    }
  };

  const stop = (e: Event) => {
    e.stopPropagation();
  };

  const randomString = `editor-${Math.random()}`;
  const navigator = useNavigate();
  const clickInput = () => {
    if (!editable() && props.link) {
      navigator(props.link);
    }
  };

  const [isFocusInput, setIsFocusInput] = createSignal(false);
  return (
    <div
      class="flex items-center gap-4 w-full"
      oninput={stop}
      onclick={stop}
      onkeydown={stop}
    >
      <div class="relative flex-1">
        <input
          id={randomString}
          class={`flex-1 ${props.link ? "cursor-pointer" : ""} ${
            props.inputClass ?? ""
          }`}
          value={text()}
          oninput={input}
          readOnly={!editable()}
          onclick={clickInput}
          onfocus={() => setIsFocusInput(editable() && true)}
          onfocusout={() => setIsFocusInput(false)}
        />
        <div
          classList={{
            "scale-0": !isFocusInput(),
            "scale-100": isFocusInput(),
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
      <button
        onclick={clickStartOrConfirm}
        class="opacity-50 hover:opacity-80 transition-all"
      >
        <Switch>
          <Match when={editable()}>
            <AiOutlineCheck size="1.2rem" />
          </Match>
          <Match when={!editable()}>
            <AiOutlineEdit size="1.2rem" />
          </Match>
        </Switch>
      </button>
      <button
        onclick={cancelEdit}
        classList={{ "opacity-100": editable(), "opacity-0": !editable() }}
      >
        <AiOutlineClose
          class="text-error opacity-50 hover:opacity-80 transition-all"
          size="1.2rem"
        />
      </button>
    </div>
  );
};

export default Editor;
