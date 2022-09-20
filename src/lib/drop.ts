import { useLocation, useParams } from "@solidjs/router";
import { appWindow } from "@tauri-apps/api/window";
import { createSignal } from "solid-js";
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

  appWindow.onFileDropEvent(async (ev) => {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      if (ev.payload.type !== "drop" || loading()) {
        return;
      }
      setLoading(true);
      setFilePaths(ev.payload.paths);
      if (isArtistPage()) {
        const artist = await commandNullWrapper(commandGetArtist)(params["id"]);
        if (artist) {
          await commandNullWrapper(commandImportFile)({
            artistName: artist.name,
            filePaths: filePaths(),
          });
          refetch();
        }
      } else {
        setIsOpenFileDialog(true);
      }
      setLoading(false);
    }, 500);
  });

  return { isOpenFileDialog, closeFileDialog, refetch, filePaths };
};

export default useDrop;
