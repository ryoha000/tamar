import { Component } from "solid-js";
import Header from "../components/TopPage/Header";
import Work from "../components/TopPage/Work";
import MasonryWrapper from "../components/UI/MasonryWrapper";
import { useStore } from "../lib/store";

const TopPage: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  const { works } = store;
  const debugWorks = () => {
    const res = [];
    for (let i = 0; i < 10; i++) {
      res.push(...works());
    }
    return res;
  };
  return (
    <div class="flex p-4 pt-14">
      <Header />
      <div>
        <MasonryWrapper each={debugWorks()}>
          {(work, i) => <Work work={work} />}
        </MasonryWrapper>
      </div>
    </div>
  );
};

export default TopPage;
