import { useParams } from "@solidjs/router";
import { Component, createResource, Show } from "solid-js"
import Header from "../components/TopPage/Header";
import Artist from "../components/UI/Artist";
import { commandGetArtist } from "../lib/commands";

const ArtistPage: Component = () => {
  const params = useParams()
  const [artist, { refetch }] = createResource(() => params["id"], commandGetArtist, { initialValue: null })

  return (
    <Show when={artist()}>
      <div class="flex p-4 pt-14">
        <Header />
        <Artist artist={artist()!} refetch={refetch} />
      </div>
    </Show>
  );
};

export default ArtistPage;
