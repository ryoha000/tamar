import { useNavigate } from "@solidjs/router";
import { createResource, createSignal, Setter } from "solid-js";
import { commandGetInitialSuggest, commandGetSuggest, commandUseSuggest } from "../../../lib/commands";
import useInputList from "../../../lib/inputList";
import { Suggest, Tag } from "../../../lib/types";

const INITIAL_SUGGEST: Suggest = { tags: [], artists: [] };
const ARTIST_SUGGEST_TYPE = 0
const TAG_SUGGEST_TYPE = 1

export interface UseSuggestProps {
  setText: Setter<string>;
  setTags: Setter<Tag[]>;
}

const useSuggest = (props: UseSuggestProps) => {
  const navigator = useNavigate()
  const [tempText, setTempText] = createSignal("");

  const fetchOption = async (text: string) => {
    if (text.length === 0) {
      return await commandGetInitialSuggest(15)
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
      opts.push({ id: artist.id, type: "artist", value: artist.name });
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

    if (option.type === "artist") {
      commandUseSuggest({ value_id: option.id, value_type: ARTIST_SUGGEST_TYPE })
      navigator(`/artist/${option.id}`)
    }

    if (option.type === "tag") {
      commandUseSuggest({ value_id: option.id, value_type: TAG_SUGGEST_TYPE })
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

export default useSuggest;
