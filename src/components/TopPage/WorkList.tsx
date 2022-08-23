import { Component, createResource } from "solid-js";
import Work from "./Work";
import MasonryWrapper from "../UI/MasonryWrapper";
import { useStore } from "../../lib/store";
import { commandSearchWork } from "../../lib/commands";

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

  const debugWorks = () => {
    const res = [];
    for (let i = 0; i < 10; i++) {
      res.push(...works());
    }
    return res;
  };
  return (
    <MasonryWrapper each={debugWorks()}>
      {(work, i) => <Work work={work} />}
    </MasonryWrapper>
  );
};

export default WorkList;
