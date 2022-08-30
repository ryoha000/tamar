import { createResource, createSignal } from "solid-js";
import { errorToast } from "../../../lib/toast";

export interface EditProps {
  initialText: string;
  command: (text: string) => Promise<void>;
  refetch: () => void;
  fetchSuggests?: (text: string) => Promise<string[]>;
  initialSuggests?: string[];
}

const useEdit = (props: EditProps) => {
  const [text, setText] = createSignal(props.initialText);
  const [editable, setEditable] = createSignal(false);

  const fetchSuggestsWrapper = async (s: string) => {
    if (!props.fetchSuggests) {
      return [];
    }
    return await props.fetchSuggests(s);
  };
  const [suggests] = createResource(text, fetchSuggestsWrapper, {
    initialValue: props.initialSuggests ?? [],
  });

  const startEdit = () => setEditable(true);
  const input = (e: InputEvent) => {
    if (!e.target || !(e.target instanceof HTMLInputElement)) {
      return;
    }
    setText(e.target.value);
  };
  const confirmEdit = async () => {
    try {
      await props.command(text());
      props.refetch();
      setEditable(false);
    } catch (e) {
      if (e instanceof Error) {
        errorToast(e.message)
      }
    }
  };
  const cancelEdit = () => {
    setText(props.initialText);
    setEditable(false);
  };
  const keydownInput = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      confirmEdit();
    }
    console.log(e.key);
  };

  return {
    startEdit,
    input,
    editable,
    suggests,
    confirmEdit,
    cancelEdit,
    text,
  };
};

export default useEdit;
