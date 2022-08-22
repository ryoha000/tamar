import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, For, Show } from "solid-js";
import usePath from "../../lib/path";
import { Work as WorkI } from "../../lib/types";
import Tag from "../UI/Tag";

interface Props {
  work: WorkI;
}

const Work: Component<Props> = (props) => {
  const images = () => props.work.paths.map((v) => convertFileSrc(v));

  return (
    <div class="flex flex-col items-center content-center gap-2 rounded hover:bg-secondary transition-all p-3 cursor-pointer">
      <Show when={images().length}>
        <img class="object-contain" src={images()[0]} />
      </Show>
      {/* <div>{props.work.title}</div>
      <div>{props.work.artist.name}</div>
      <For each={props.work.tags}>
        {(tag, i) => (
          <div class="flex">
            <Tag tag={tag} />
          </div>
        )}
      </For> */}
    </div>
  );
};

export default Work;
