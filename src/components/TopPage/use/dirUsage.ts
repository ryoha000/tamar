import { fs, path } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor, createEffect, createSignal } from "solid-js";
import { command_import_directory } from "../../../lib/commands";
import { DirPathInfo } from "./exploreDir";

export interface Usages {
  [maxDeps: number]: {
    [deps: number]: DepsUsageKind;
  };
}

export const DEPS_USAGE = ["作品名", "作者名", "タグ", "無視する"] as const;
export type DepsUsageKind = typeof DEPS_USAGE[number];

const IMAGE_EXTENTION = ["gif", "jpg", "jpeg", "jpe", "jfif", "png", "webp"];

interface DirDepsLengthKind {
  deps: number;
  index: number;
}

const useDirUsage = (
  paths: Accessor<DirPathInfo[]>,
  targetMaxDeps: Accessor<number>
) => {
  const [usages, setUsages] = createSignal<Usages>({});
  const [sampleSrc, setSampleSrc] = createSignal("");

  createEffect(async () => {
    if (paths().length === 0) {
      return;
    }
    const index = dirDepsLengthKind().find(
      (v) => v.deps === targetMaxDeps()
    )?.index;
    if (index === undefined) {
      return;
    }

    const entries = await fs.readDir(paths()[index].path);
    for (const entry of entries) {
      const ext = await path.extname(entry.path);
      if (IMAGE_EXTENTION.includes(ext.toLowerCase())) {
        return setSampleSrc(convertFileSrc(entry.path));
      }
    }
    return;
  });

  const dirDepsLengthKind = () => {
    if (paths().length === 0) {
      return [];
    }
    const existDeps = new Set<number>();

    const res: DirDepsLengthKind[] = [];
    for (let i = 0; i < paths().length; i++) {
      const len = paths()[i].dirDeps.length;
      if (!existDeps.has(len)) {
        res.push({ deps: len, index: i });
        existDeps.add(len);
      }
    }
    res.sort((a, b) => a.deps - b.deps);
    return res;
  };

  const dirDepsLengthKindOnlyDeps = () => {
    return dirDepsLengthKind().map((v) => `${v.deps}`);
  };

  createEffect(() => {
    const initialUsage: Usages = {};
    dirDepsLengthKind().forEach((v) => {
      // TODO: いい感じにinitialを決定する
      initialUsage[v.deps] = {};
      paths()[v.index].dirDeps.forEach((dep) => {
        initialUsage[v.deps][dep.deps] = "無視する";
      });
    });

    setUsages(initialUsage);
  });

  const eachDepsSample = () => {
    if (paths().length === 0) {
      return [];
    }
    const index = dirDepsLengthKind().find(
      (v) => v.deps === targetMaxDeps()
    )?.index;
    if (index === undefined) {
      return [];
    }

    return paths()[index].dirDeps;
  };

  const getUsage = (deps: number) => {
    return usages()[targetMaxDeps()][deps];
  };

  const setUsage = (deps: number, usage: DepsUsageKind) => {
    const _usages = usages();
    _usages[targetMaxDeps()][deps] = usage;
    setUsages(_usages);
  };

  const confirm = async () => {
    await command_import_directory(paths(), usages());
  };

  return {
    eachDepsSample,
    getUsage,
    setUsage,
    sampleSrc,
    dirDepsLengthKindOnlyDeps,
    confirm,
  };
};

export default useDirUsage;