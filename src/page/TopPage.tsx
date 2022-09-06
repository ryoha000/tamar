import { Component, Match, Show, Switch } from "solid-js";
import ArtistList from "../components/TopPage/ArtistList";
import FileImportDialog from "../components/TopPage/FileImportDialog";
import Header from "../components/TopPage/Header";
import WorkList from "../components/TopPage/WorkList";
import useDrop from "../lib/drop";
import { useStore } from "../lib/store";

const TopPage: Component = () => {
  const store = useStore();
  const { isOpenFileDialog, closeFileDialog, refetch, filePaths } = useDrop();

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

        <FileImportDialog
          isOpen={isOpenFileDialog()}
          close={closeFileDialog}
          refetch={refetch}
          filePaths={filePaths()}
        />
      </div>
    </Show>
  );
};

export default TopPage;
