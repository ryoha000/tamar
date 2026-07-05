import { useLocation, useParams } from "@solidjs/router";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal, onCleanup } from "solid-js";
import { commandGetArtist, commandImportFile } from "./commands";
import { useStore } from "./store";
import { commandNullWrapper } from "./toast";

const useDrop = () => {
  const [isOpenFileDialog, setIsOpenFileDialog] = createSignal(false);
  const closeFileDialog = () => {
    setIsOpenFileDialog(false);
    setFilePaths([]);
  };
  const [filePaths, setFilePaths] = createSignal<string[]>([]);

  const store = useStore();

  const refetch = () => {
    if (store) {
      store.refetch();
    }
  };

  const location = useLocation();
  const isArtistPage = () => location.pathname.startsWith("/artist");
  const params = useParams();

  const [loading, setLoading] = createSignal(false);

  let timer = 0;
  let unlistenFileDropEvent: (() => void) | undefined;
  let disposed = false;

  appWindow.onFileDropEvent(async (ev) => {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      if (ev.payload.type !== "drop" || loading()) {
        return;
      }
      setLoading(true);
      const droppedFilePaths = ev.payload.paths;
      setFilePaths(droppedFilePaths);
      const artistId = params["id"];
      if (isArtistPage() && artistId) {
        const artist = await commandNullWrapper(commandGetArtist)(artistId);
        if (artist) {
          await commandNullWrapper(commandImportFile)({
            artistName: artist.name,
            filePaths: droppedFilePaths,
          });
          refetch();
        }
      } else {
        setIsOpenFileDialog(true);
      }
      setLoading(false);
    }, 500);
  }).then((unlisten) => {
    if (disposed) {
      unlisten();
      return;
    }
    unlistenFileDropEvent = unlisten;
  });

  onCleanup(() => {
    disposed = true;
    clearTimeout(timer);
    unlistenFileDropEvent?.();
  });

  return { isOpenFileDialog, closeFileDialog, refetch, filePaths };
};

export default useDrop;
