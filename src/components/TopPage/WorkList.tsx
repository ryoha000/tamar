import { Component, createResource, createSignal } from "solid-js";
import Work from "./Work";
import MasonryWrapper from "../UI/MasonryWrapper";
import { useStore } from "../../lib/store";
import { commandSearchWork } from "../../lib/commands";
import ScrollObserber from "../UI/ScrollObserver";
import { SEARCH_LIMIT } from "../../lib/option";

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

  const [loading, setLoading] = createSignal(false);
  const [enableScroll, setEnableScroll] = createSignal(true);
  const [offset, setOffset] = createSignal(0);
  const fetechMore = async () => {
    setLoading(true);
    setOffset((prev) => prev + SEARCH_LIMIT);
    const req = { ...store.searchRequest(), offset: offset() };
    const more = await commandSearchWork(req);
    if (more.length === 0) {
      console.log("全部見た");
      // setEnableScroll(false);
    }
    mutate((prev) => [...prev, ...more]);
    setLoading(false);
  };
  return (
    <div>
      <MasonryWrapper each={works()}>
        {(work, i) => <Work work={work} />}
      </MasonryWrapper>
      <ScrollObserber
        isActiveObserver={enableScroll()}
        isLoading={loading() || works().length === 0}
        onIntersect={fetechMore}
      />
    </div>
  );
};

export default WorkList;
