import { Component, createResource, Index } from "solid-js";
import { useStore } from "../../lib/store";
import { commandSearchArtist } from "../../lib/commands";
import Artist from "../UI/Artist";

const ArtistList: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  const [artists, { mutate, refetch }] = createResource(
    store.searchRequest,
    commandSearchArtist,
    {
      initialValue: [],
    }
  );

  return (
    <div class="flex flex-col items-center w-full gap-4">
      <Index each={artists()}>
        {(artist, i) => <Artist artist={artist()} refetch={refetch} />}
      </Index>
    </div>
  );
};

export default ArtistList;
