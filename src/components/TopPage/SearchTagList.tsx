import { Component, For } from "solid-js";
import { Tag as TagI } from "../../lib/types";
import HorizontalScroller from "../UI/HorizontalScroller";
import Tag from "../UI/Tag";

interface Props {
  tags: TagI[];
}
const SearchTagList: Component<Props> = (props) => {
  const debugTags = () => {
    const names = [
      "aaaaaa",
      "bbb",
      "blue archive",
      "blue archive",
      "blue archive",
    ];
    const res: TagI[] = [];
    for (const name of names) {
      res.push({ id: `${name}-id`, name, updatedAt: "" });
    }
    return res;
  };

  return (
    <div class="flex h-full min-w-0 max-w-search-tags">
      <HorizontalScroller
        scrollStep={150}
        isGradientFader={false}
        iconSize="sm"
      >
        <div class="flex items-center">
          <For each={debugTags()}>{(tag, i) => <Tag tag={tag} />}</For>
        </div>
      </HorizontalScroller>
    </div>
  );
};

export default SearchTagList;
