import { createSignal, Setter } from "solid-js";

const useSuggest = (props: { setText: Setter<string> }) => {
  // https://stackoverflow.com/a/65073572
  let eventSource: "input" | "list" | null = null;

  const [tags, setTags] = createSignal<string[]>([]);

  const keydown = (e: KeyboardEvent) => {
    eventSource = e.key ? "input" : "list";
  };
  const input = (e: InputEvent) => {
    if (!e.target || !(e.target instanceof HTMLInputElement)) {
      return;
    }
    const value = e.target.value;
    if (eventSource === "list") {
      alert("CLICKED! " + value);
    }
  };
  return { keydown, input };
};

export default useSuggest;
