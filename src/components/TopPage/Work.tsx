import { Link } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, For, Show } from "solid-js";
import { useStore } from "../../lib/store";
import { Work as WorkI } from "../../lib/types";

interface Props {
  work: WorkI;
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

  return (
    <div class="hover:scale-110 hover:shadow-md transition-all cursor-pointer">
      <Show when={imageSrc()}>
        <Link
          href={`/work/${props.work.id}/${workPageMap.get(props.work.id) ?? 0}`}
        >
          <img class="object-contain" src={imageSrc()} />
        </Link>
      </Show>
    </div>
  );
};

export default Work;
