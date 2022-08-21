import { invoke } from "@tauri-apps/api";
import { Usages } from "../components/TopPage/use/dirUsage";
import { DirPathInfo } from "../components/TopPage/use/exploreDir";

export const commandImportDirectory = async (
  dirPathInfos: DirPathInfo[],
  usages: Usages
) => {
  await invoke<null>("import_directory", { dirPathInfos, usages });
};
