import { Link } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, For, Show } from "solid-js";
import { Work as WorkI } from "../../lib/types";

interface Props {
  work: WorkI;
}

const Work: Component<Props> = (props) => {
  const images = () => props.work.paths.map((v) => convertFileSrc(v));

  return (
    <div class="hover:scale-110 transition-all cursor-pointer">
      <Show when={images().length}>
        <Link href={`/work/${props.work.id}/view`}>
          <img class="object-contain" src={images()[0]} />
        </Link>
      </Show>
    </div>
  );
};

export default Work;
