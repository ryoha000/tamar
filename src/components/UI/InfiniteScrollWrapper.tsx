import { Component, createSignal, ParentComponent } from "solid-js";
import { SEARCH_LIMIT } from "../../lib/option";
import { Artist, SearchWorkRequest, WorkSummary } from "../../lib/types";
import ScrollObserber from "./ScrollObserver";

type BaseProps<T> = Props<T> & { step: number };

interface Props<T> {
  command: (req: SearchWorkRequest) => Promise<T[]>;
  mutate: (value: (prev: T[]) => T[]) => T[];
  req: SearchWorkRequest;
  isInitialEmpty: boolean;
  initOffset: number;
}

// generics が使えないので
type TypedBaseProps = BaseProps<WorkSummary | Artist>;
type TypedProps = Props<WorkSummary | Artist>;

const InfiniteScrollBase: Component<TypedBaseProps> = (props) => {
  const [loading, setLoading] = createSignal(false);
  const [enableScroll, setEnableScroll] = createSignal(true);
  const [offset, setOffset] = createSignal(props.initOffset);
  const fetechMore = async () => {
    setOffset((prev) => prev + props.step);
    if (offset() < 0) {
      return;
    }
    setLoading(true);
    const req = { ...props.req, offset: offset() };
    const more = await props.command(req);
    if (more.length === 0) {
      console.log("全部見た");
      setEnableScroll(false);
    }
    if (props.step < 0) {
      props.mutate((prev) => [...more, ...prev]); // MEMO: offsetが切りよくないと同じのが入る
    } else {
      props.mutate((prev) => [...prev, ...more]);
    }
    setLoading(false);
  };

  return (
    <ScrollObserber
      isActiveObserver={enableScroll() && !props.isInitialEmpty}
      isLoading={loading()}
      onIntersect={fetechMore}
    />
  );
};

const InfiniteScroll: ParentComponent<TypedProps> = (props) => {
  return (
    <div class="w-full">
      <InfiniteScrollBase
        command={props.command}
        mutate={props.mutate}
        req={props.req}
        isInitialEmpty={props.isInitialEmpty}
        initOffset={props.initOffset}
        step={SEARCH_LIMIT * -1}
      />
      {props.children}
      <InfiniteScrollBase
        command={props.command}
        mutate={props.mutate}
        req={props.req}
        isInitialEmpty={props.isInitialEmpty}
        initOffset={props.initOffset}
        step={SEARCH_LIMIT}
      />
    </div>
  );
};

export default InfiniteScroll;
