import { Component, Match, Show, Switch } from "solid-js";
import ArtistList from "../components/TopPage/ArtistList";
import Header from "../components/TopPage/Header";
import WorkList from "../components/TopPage/WorkList";
import { useStore } from "../lib/store";

const TopPage: Component = () => {
  const store = useStore();

  return (
    <Show when={store}>
      <div class="flex p-4 pt-14">
        <Header />
        <Switch>
          <Match when={!store!.isFilterArtist()}>
            <WorkList />
          </Match>
          <Match when={store!.isFilterArtist()}>
            <ArtistList />
          </Match>
        </Switch>
      </div>
    </Show>
  );
};

export default TopPage;
