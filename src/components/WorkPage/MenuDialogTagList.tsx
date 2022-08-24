import { AiOutlinePlus } from "solid-icons/ai";
import {
  Component,
  createResource,
  createSignal,
  For,
  onMount,
} from "solid-js";
import { commandGetTagSuggest, commandSelectTag } from "../../lib/commands";
import { Tag as TagI } from "../../lib/types";
import Tag from "../UI/Tag";

interface Props {
  tags: TagI[];
}

const MenuDialogTagList: Component<Props> = (props) => {
  const removeTag = (id: string) => {};

  const [newTagText, setNewTagText] = createSignal("");
  const onchange = (
    e: Event & { currentTarget: HTMLInputElement; target: Element }
  ) => {
    setNewTagText(e.currentTarget.value);
  };

  const [suggests, { mutate }] = createResource(
    newTagText,
    commandGetTagSuggest,
    { initialValue: [] }
  );
  onMount(async () => {
    const initialSuggest = await commandSelectTag(50);
    mutate(initialSuggest);
  });

  return (
    <div>
      <div class="flex flex-wrap items-center gap-2 w-full">
        <For each={props.tags}>
          {(tag, i) => (
            <Tag tag={tag} isCloseIcon={true} close={() => removeTag(tag.id)} />
          )}
        </For>
      </div>
      <div class="w-full flex items-center gap-4">
        <input
          id="menudialogtaglist-input"
          class="flex-1 border-b border-text focus:border-secondary transition-all"
          value={newTagText()}
          onchange={onchange}
        />
        <datalist id="menudialogtaglist-input">
          <For each={suggests()}>
            {(suggest, i) => <option>{suggest.name}</option>}
          </For>
        </datalist>
        <button>
          <AiOutlinePlus size="1.5rem" />
        </button>
      </div>
    </div>
  );
};

export default MenuDialogTagList;
