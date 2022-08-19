import { invoke } from "@tauri-apps/api";
import { Usages } from "../components/TopPage/use/dirUsage";
import { DirPathInfo } from "../components/TopPage/use/exploreDir";

export const command_import_directory = async (
  dirPathInfos: DirPathInfo[],
  usages: Usages[]
) => {
  console.log({ dirPathInfos, usages });
  await invoke("import_directory", { dirPathInfos, usages });
};
