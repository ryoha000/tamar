import { Link } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, For, Show } from "solid-js";
import { Work as WorkI } from "../../lib/types";

interface Props {
  work: WorkI;
}

const Work: Component<Props> = (props) => {
  const imageSrc = () => {
    if (!props.work.paths.length) {
      return "";
    }
    return convertFileSrc(props.work.paths[0]);
  };

  return (
    <div class="hover:scale-110 transition-all cursor-pointer">
      <Show when={imageSrc()}>
        <Link href={`/work/${props.work.id}/${1}`}>
          <img class="object-contain" src={imageSrc()} />
        </Link>
      </Show>
    </div>
  );
};

export default Work;
