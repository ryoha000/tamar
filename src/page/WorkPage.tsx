import { useParams } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Component, onMount } from "solid-js";
import Header from "../components/TopPage/Header";
import { useStore } from "../lib/store";

const WorkPage: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }
  const params = useParams();

  onMount(() => {
    console.log("onMount WorkPage");
  });

  const { works } = store;
  const work = () => {
    const workId = params["id"];
    return works().find((v) => v.id === workId)!;
  };
  const imageSrc = () => {
    const sortedPaths = [...work().paths];
    sortedPaths.sort();
    return sortedPaths.map((v) => convertFileSrc(v));
  };

  return (
    <div class="flex">
      <div class="h-12 bg-opacity-50 bg-slate-500 fixed z-header w-full">
        ここにヘッダー {`page: ${params["page"]}`}
      </div>
      <img
        src={imageSrc()[+params["page"]]}
        class="w-screen h-screen object-contain"
      ></img>
      <div>WorkPage</div>
    </div>
  );
};

export default WorkPage;
