import { Component, createResource, onMount } from "solid-js";
import Work from "./Work";
import MasonryWrapper from "../UI/MasonryWrapper";
import { useStore } from "../../lib/store";
import { commandSearchWork } from "../../lib/commands";
import InfiniteScroll from "../UI/InfiniteScrollWrapper";

const WorkList: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  const [works, { mutate, refetch }] = createResource(
    store.searchRequest,
    commandSearchWork,
    {
      initialValue: [],
    }
  );

  onMount(() => {
    store.refetch = refetch;
  });

  return (
    <InfiniteScroll
      command={commandSearchWork}
      mutate={mutate}
      req={store.searchRequest()}
      isInitialEmpty={works().length === 0}
      initOffset={store.searchRequest().offset}
    >
      <MasonryWrapper each={works()}>
        {(work, i) => <Work work={work} />}
      </MasonryWrapper>
    </InfiniteScroll>
  );
};

export default WorkList;
