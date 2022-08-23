import { Link } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, Show } from "solid-js";
import { useStore } from "../../lib/store";
import { Work } from "../../lib/types";

interface Props {
  work: Work;
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

  return (
    <div class="w-full h-full hover:scale-110 hover:shadow-md focus-within:scale-110 focus-within:shadow-md transition-all cursor-pointer">
      <Show when={imageSrc()}>
        <Link
          tabIndex={0}
          href={`/work/${props.work.id}/${workPageMap.get(props.work.id) ?? 0}`}
        >
          <img class="object-cover w-full h-full rounded-lg" src={imageSrc()} />
        </Link>
      </Show>
    </div>
  );
};

export default ArtistWork;
