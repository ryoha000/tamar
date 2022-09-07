import { useNavigate } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, Show } from "solid-js";
import { SEARCH_LIMIT } from "../../lib/option";
import { useStore } from "../../lib/store";
import { Work } from "../../lib/types";

interface Props {
  work: Work;
  index?: number;
}

const ArtistWork: Component<Props> = (props) => {
  const store = useStore();
  if (!store) {
    return null;
  }
  const { workPageMap } = store;

  const imageSrc = () => {
    if (!props.work.paths.length) {
      return "";
    }
    return convertFileSrc(props.work.paths[0]);
  };

  const navigator = useNavigate();
  const goWorkPage = () => {
    if (props.index) {
      store.setOffset(Math.floor(props.index / SEARCH_LIMIT) * SEARCH_LIMIT);
    }
    navigator(`/work/${props.work.id}/${workPageMap.get(props.work.id) ?? 0}`);
  };
  const keydown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      goWorkPage();
    }
  };

  return (
    <div class="w-full h-full hover:scale-110 hover:shadow-md focus-within:scale-110 focus-within:shadow-md transition-all cursor-pointer">
      <Show when={imageSrc()}>
        <div tabIndex={0} onclick={goWorkPage} onkeydown={keydown}>
          <img
            loading="lazy"
            decoding="async"
            class="object-cover w-full h-full rounded-lg"
            src={imageSrc()}
          />
        </div>
      </Show>
    </div>
  );
};

export default ArtistWork;
