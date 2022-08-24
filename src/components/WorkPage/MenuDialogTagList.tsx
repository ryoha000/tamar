import { AiOutlinePlus } from "solid-icons/ai";
import { Component, createResource, createSignal, For, Show } from "solid-js";
import {
  commandAttachTagByName,
  commandDetachTag,
  commandGetTagSuggest,
} from "../../lib/commands";
import useInputList from "../../lib/inputList";
import { Tag as TagI } from "../../lib/types";
import Tag from "../UI/Tag";

interface Props {
  workId: string;
  tags: TagI[];
  refetch: () => void;
}

const MenuDialogTagList: Component<Props> = (props) => {
  const removeTag = async (id: string) => {
    await commandDetachTag(props.workId, id);
    props.refetch();
  };

  const [newTagText, setNewTagText] = createSignal("");

  const inputCallback = (ele: HTMLInputElement) => {
    setNewTagText(ele.value);
  };
  const listCallback = (ele: HTMLInputElement) => {
    setNewTagText(ele.value);
    attachTag(null);
  };
  const { input, keydown } = useInputList(inputCallback, listCallback);

  const [suggests] = createResource(newTagText, commandGetTagSuggest, {
    initialValue: [],
  });

  const attachTag = async (e: Event | null) => {
    e?.preventDefault();
    await commandAttachTagByName(props.workId, newTagText());
    props.refetch();
    setNewTagText("");
  };

  return (
    <div class="flex flex-col gap-4">
      <div class="flex flex-wrap items-center gap-2 w-full">
        <For each={props.tags}>
          {(tag, i) => <Tag tag={tag} close={() => removeTag(tag.id)} />}
        </For>
      </div>
      <form onsubmit={attachTag} class="w-full flex items-center gap-4">
        <input
          list="menudialogtaglist"
          type="text"
          class="flex-1 border-b border-text focus:border-secondary transition-all max-w-xs"
          value={newTagText()}
          oninput={input}
          onkeydown={keydown}
        />
        <datalist id="menudialogtaglist">
          <For each={suggests()}>
            {(suggest, i) => (
              <Show when={!props.tags.find((v) => v.id === suggest.id)}>
                <option>{suggest.name}</option>
              </Show>
            )}
          </For>
        </datalist>
        <button type="submit">
          <AiOutlinePlus size="1.5rem" />
        </button>
      </form>
    </div>
  );
};

export default MenuDialogTagList;
