import { useParams } from "@solidjs/router";
import { Component, onMount } from "solid-js";
import Header from "../components/WorkPage/Header";
import {
  NextOverlay,
  PrevOverlay,
} from "../components/WorkPage/NavigationOverlay";
import usePage from "../components/WorkPage/use/page";
import { useStore } from "../lib/store";

const WorkPage: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }
  const params = useParams();

  onMount(() => {
    console.log("onMount WorkPage");
    // TODO: 閲覧履歴を insert する
  });

  const { works, workPageMap } = store;
  const work = () => {
    const workId = params["id"];
    // TODO: ちゃんとfetchする(F5押したときに困る)
    const v = works().find((v) => v.id === workId);
    return v ?? null;
  };

  const { imageSrc, next, prev, keyDown } = usePage(work, workPageMap);

  return (
    <div class="flex" onkeydown={keyDown}>
      <Header />
      <img src={imageSrc()} class="w-screen h-screen object-contain"></img>
      <NextOverlay navigate={next} />
      <PrevOverlay navigate={prev} />
    </div>
  );
};

export default WorkPage;
