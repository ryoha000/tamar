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
    <div class="flex flex-col items-center content-center gap-2 hover:scale-110 transition-all p-3 cursor-pointer">
      <Show when={images().length}>
        <img class="object-contain" src={images()[0]} />
      </Show>
    </div>
  );
};

export default Work;
