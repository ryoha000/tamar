import { fs, path } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor, createEffect, createSignal } from "solid-js";
import { DirPathInfo } from "./exploreDir";

interface DepsUsage {
  deps: number;
  usage: DepsUsageKind;
}

export const DEPS_USAGE = ["作品名", "作者名", "タグ", "無視する"] as const;
export type DepsUsageKind = typeof DEPS_USAGE[number];

const IMAGE_EXTENTION = ["gif", "jpg", "jpeg", "jpe", "jfif", "png", "webp"];

const useDirUsage = (paths: Accessor<DirPathInfo[]>) => {
  const [usages, setUsages] = createSignal<DepsUsage[]>([]);
  const [sampleSrc, setSampleSrc] = createSignal("");

  createEffect(async () => {
    if (paths().length === 0) {
      return;
    }
    const maxIndex = maxDirDepsLengthIndex();

    const entries = await fs.readDir(paths()[maxIndex].path);
    for (const entry of entries) {
      const ext = await path.extname(entry.path);
      if (IMAGE_EXTENTION.includes(ext.toLowerCase())) {
        return setSampleSrc(convertFileSrc(entry.path));
      }
    }
    return;
  });

  const maxDirDepsLengthIndex = () => {
    let maxIndex = 0;
    for (let i = 0; i < paths().length; i++) {
      if (paths()[maxIndex].dirDeps.length < paths()[i].dirDeps.length) {
        maxIndex = i;
      }
    }
    return maxIndex;
  };

  const eachDepsSample = () => {
    if (paths().length === 0) {
      return [];
    }
    const maxIndex = maxDirDepsLengthIndex();

    // TODO: いい感じにinitialを決定する
    setUsages(
      paths()[maxIndex].dirDeps.map(
        (v) =>
          ({
            deps: v.deps,
            usage: "無視する",
          } as DepsUsage)
      )
    );

    return paths()[maxIndex].dirDeps;
  };

  const getUsage = (deps: number) => {
    return usages().find((v) => v.deps === deps)!.usage;
  };

  const setUsage = (deps: number, usage: DepsUsageKind) => {
    setUsages(
      usages().map((v) => {
        if (v.deps !== deps) {
          return v;
        }
        return { deps, usage };
      })
    );
  };

  return { eachDepsSample, getUsage, setUsage, sampleSrc };
};

export default useDirUsage;
