import { useLocation, useParams } from "@solidjs/router"
import { appWindow } from "@tauri-apps/api/window"
import { createSignal } from "solid-js"
import { commandGetArtist, commandImportFile } from "./commands"
import { useStore } from "./store"

const useDrop = () => {
  const [isOpenFileDialog, setIsOpenFileDialog] = createSignal(false)
  const closeFileDialog = () => setIsOpenFileDialog(false)
  const [filePaths, setFilePaths] = createSignal<string[]>([])

  const store = useStore()

  const refetch = () => {
    if (store) {
      store.refetch()
    }
  }
  
  const location = useLocation();
  const isArtistPage = () => location.pathname.startsWith("/artist");
  const params = useParams();

  appWindow.onFileDropEvent(async (ev) => {
    if(ev.payload.type !== "drop"){
      return
    }
    if (isArtistPage()) {
      const artist = await commandGetArtist(params["id"])
      await commandImportFile({ artistName: artist.name, filePaths: filePaths() })
      refetch()
      return
    }
    setFilePaths(ev.payload.paths)
    setIsOpenFileDialog(true)
  })

  return { isOpenFileDialog, closeFileDialog, refetch, filePaths }
}

export default useDrop
