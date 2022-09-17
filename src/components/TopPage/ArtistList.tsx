import { Component, createResource, Index, onMount } from "solid-js";
import { useStore } from "../../lib/store";
import { commandSearchArtist } from "../../lib/commands";
import Artist from "../UI/Artist";
import InfiniteScroll from "../UI/InfiniteScrollWrapper";
import { commandArrayWrapper } from "../../lib/toast";

const ArtistList: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  const [artists, { mutate, refetch }] = createResource(
    store.searchRequest,
    commandArrayWrapper(commandSearchArtist),
    {
      initialValue: [],
    }
  );

  onMount(() => {
    store.refetch = () => {
      mutate([]);
      refetch();
    };
  });

  return (
    <InfiniteScroll
      command={commandArrayWrapper(commandSearchArtist)}
      mutate={mutate}
      req={store.searchRequest()}
      isInitialEmpty={artists().length === 0}
      initOffset={store.searchRequest().offset}
    >
      <div class="flex flex-col items-center w-full gap-4">
        <Index each={artists()}>
          {(artist, i) => (
            <Artist artist={artist()} refetch={refetch} index={i} />
          )}
        </Index>
      </div>
    </InfiniteScroll>
  );
};

export default ArtistList;
