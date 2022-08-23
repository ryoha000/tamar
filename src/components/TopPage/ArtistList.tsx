import { Component, createResource } from "solid-js";
import Work from "./Work";
import MasonryWrapper from "../UI/MasonryWrapper";
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

  const debugArtists = () => {
    const res = [];
    for (let i = 0; i < 10; i++) {
      res.push(...artists());
    }
    return res;
  };
  return (
    <MasonryWrapper each={debugArtists()}>
      {(artist, i) => <Artist artist={artist} />}
    </MasonryWrapper>
  );
};

export default ArtistList;
