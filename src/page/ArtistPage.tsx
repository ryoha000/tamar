import { useParams } from "@solidjs/router";
import { Component, createResource, onMount, Show } from "solid-js";
import FileImportDialog from "../components/TopPage/FileImportDialog";
import Header from "../components/TopPage/Header";
import Artist from "../components/UI/Artist";
import { commandGetArtist } from "../lib/commands";
import useDrop from "../lib/drop";
import { useStore } from "../lib/store";
import { commandWrapper } from "../lib/toast";

const ArtistPage: Component = () => {
  const params = useParams();
  const [artist, { refetch, mutate }] = createResource(
    () => params["id"],
    commandWrapper(commandGetArtist),
    {
      initialValue: null,
    }
  );

  const store = useStore();
  onMount(() => {
    if (store) {
      store.refetch = () => {
        mutate(null);
        refetch();
      };
    }
  });

  const { isOpenFileDialog, closeFileDialog, filePaths } = useDrop();

  return (
    <Show when={artist()}>
      <div class="flex p-4 pt-14 bg-background">
        <Header />
        <Artist artist={artist()!} refetch={refetch} />
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

export default ArtistPage;
