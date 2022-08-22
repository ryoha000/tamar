import { Component, For } from "solid-js";
import Header from "../components/TopPage/Header";
import Work from "../components/TopPage/Work";
import MasonryItem from "../components/UI/MasonryItem";
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
        <MasonryWrapper>
          {(payload: { rowHeight: number; rowGap: number }) => {
            return (
              <For each={debugWorks()}>
                {(work, i) => (
                  <MasonryItem
                    rowGap={payload.rowGap}
                    rowHeight={payload.rowHeight}
                  >
                    <Work work={work} />
                  </MasonryItem>
                )}
              </For>
            );
          }}
        </MasonryWrapper>
      </div>
    </div>
  );
};

export default TopPage;
