import { Accessor, createResource, createSignal, Setter } from "solid-js";
import { commandGetSuggest } from "../../../lib/commands";
import { Suggest, Tag } from "../../../lib/types";

const INITIAL_SUGGEST: Suggest = { tags: [], artists: [] };

export interface UseSuggestProps {
  setText: Setter<string>;
  setTags: Setter<Tag[]>;
}

const useSuggest = (props: UseSuggestProps) => {
  const [tempText, setTempText] = createSignal("");

  const fetchOption = async (text: string) => {
    if (text.length === 0) {
      // TODO: history から取得
      return INITIAL_SUGGEST;
    }
    return await commandGetSuggest(text);
  };
  const [suggest] = createResource(tempText, fetchOption, {
    initialValue: INITIAL_SUGGEST,
  });

  const optionsWithMetadata = () => {
    const opts: { id: string; type: "tag" | "artist"; value: string }[] = [];
    for (const tag of suggest().tags) {
      opts.push({ id: tag.id, type: "tag", value: tag.name });
    }
    for (const artist of suggest().artists) {
      opts.push({ id: artist.id, type: "tag", value: artist.name });
    }
    return opts;
  };

  const options = () => optionsWithMetadata().map((v) => v.value);

  const inputCallback = (ele: HTMLInputElement) => {
    const text = ele.value;
    setTempText(text);
  };
  const listCallback = (ele: HTMLInputElement) => {
    const text = ele.value;
    const option = optionsWithMetadata().find((v) => v.value === text);
    if (!option) {
      inputCallback(ele);
      return;
    }
    setTempText("");
    props.setText("");
    ele.value = "";

    // TODO: artist が選択されたときは /artist/:id に飛ばす

    if (option.type === "tag") {
      props.setTags((prev) => [
        ...prev,
        { id: option.id, name: option.value, updatedAt: "" },
      ]);
    }
  };

  const { keydown, input } = useInputList(inputCallback, listCallback);

  const change = (
    e: Event & { currentTarget: HTMLInputElement; target: Element }
  ) => {
    if (!e.target || !(e.target instanceof HTMLInputElement)) {
      return;
    }
    props.setText(e.target.value);
  };
  return { keydown, input, change, options };
};

// https://stackoverflow.com/a/65073572
const useInputList = (
  inputCallback: (e: HTMLInputElement) => void,
  listCallback: (e: HTMLInputElement) => void
) => {
  let eventSource: "input" | "list" | null = null;
  const keydown = (e: KeyboardEvent) => {
    eventSource = e.key ? "input" : "list";
  };
  const input = (e: InputEvent) => {
    if (!e.target || !(e.target instanceof HTMLInputElement)) {
      return;
    }
    if (eventSource === "list") {
      listCallback(e.target);
    } else {
      inputCallback(e.target);
    }
  };

  return { keydown, input };
};

export default useSuggest;
