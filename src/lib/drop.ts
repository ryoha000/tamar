import { appWindow } from "@tauri-apps/api/window"
import { createSignal } from "solid-js"
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

  appWindow.onFileDropEvent((ev) => {
    if(ev.payload.type !== "drop"){
      return
    }
    setFilePaths(ev.payload.paths)
    console.log(filePaths())
    setIsOpenFileDialog(true)
  })

  return { isOpenFileDialog, closeFileDialog, refetch, filePaths }
}

export default useDrop
