import { Component, For } from "solid-js";
import { Tag as TagI } from "../../lib/types";
import HorizontalScroller from "../UI/HorizontalScroller";
import Tag from "../UI/Tag";

interface Props {
  tags: TagI[];
  removeTag: (id: string) => void;
}
const SearchTagList: Component<Props> = (props) => {
  return (
    <div class="flex h-full min-w-0 max-w-search-tags">
      <HorizontalScroller
        scrollStep={150}
        isGradientFader={false}
        iconSize="sm"
      >
        <div class="flex items-center gap-2">
          <For each={props.tags}>
            {(tag, i) => (
              <Tag tag={tag} close={() => props.removeTag(tag.id)} />
            )}
          </For>
        </div>
      </HorizontalScroller>
    </div>
  );
};

export default SearchTagList;
