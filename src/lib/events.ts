import { listen } from "@tauri-apps/api/event";
import { Setter } from "solid-js";

export const listenImportDirProgress = async (cnt: Setter<number>) => {
  await listen<{ count: number }>('import_dir_progress', (event) => {
    cnt((prev) => prev + event.payload.count)
  })
}
