import { useNavigate } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, Show } from "solid-js";
import { SEARCH_LIMIT } from "../../lib/option";
import { useStore } from "../../lib/store";
import { Work as WorkI } from "../../lib/types";

interface Props {
  work: WorkI;
  index: number;
}

const Work: Component<Props> = (props) => {
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
    const offset = Math.floor(props.index / SEARCH_LIMIT) * SEARCH_LIMIT;
    store.setOffset(offset);
    const link = `/work/${props.work.id}/${
      workPageMap.get(props.work.id) ?? 0
    }`;
    navigator(link);
  };

  const keydown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      goWorkPage();
    }
  };

  return (
    <div class="hover:scale-110 hover:shadow-md focus-within:scale-110 focus-within:shadow-md transition-all cursor-pointer">
      <Show when={imageSrc()}>
        <div tabIndex={0} onclick={goWorkPage} onkeydown={keydown}>
          <img
            decoding="async"
            class="object-contain rounded"
            src={imageSrc()}
            loading="lazy"
          />
        </div>
      </Show>
    </div>
  );
};

export default Work;
