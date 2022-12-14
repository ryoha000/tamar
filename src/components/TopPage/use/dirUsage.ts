import { fs, path } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor, createEffect, createSignal } from "solid-js";
import { commandImportDirectory } from "../../../lib/commands";
import { commandNullWrapper } from "../../../lib/toast";
import { Tag, UNKNOWN_ARTIST_NAME } from "../../../lib/types";
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
  const [usages, setUsages] = createSignal<Usages>({}, { equals: false });
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

  const [ignoreDeps, setIgnoreDeps] = createSignal<{ [key: number]: boolean }>(
    {},
    { equals: false }
  );
  const getIgnoreDeps = () => {
    const deps = targetMaxDeps();
    return ignoreDeps()[deps];
  };
  const toggleIgnoreDeps = () => {
    setIgnoreDeps((prev) => {
      const deps = targetMaxDeps();
      prev[deps] = !prev[deps];
      return prev;
    });
  };

  createEffect(() => {
    const initialUsage: Usages = {};
    const initialIgnore: { [key: number]: boolean } = {};
    dirDepsLengthKind().forEach((v) => {
      initialUsage[v.deps] = {};
      initialIgnore[v.deps] = false;
      paths()[v.index].dirDeps.forEach((dep) => {
        let usage: DepsUsageKind = "無視する";
        // when >= 3. 1 => ignore, 2 => artist, ... , last => title
        // when = 2. 1 => artist, 2 => title
        // when = 1. 1 => title
        if (v.deps >= 3 && dep.deps === 2) {
          usage = "作者名";
        }
        if (v.deps === 2 && dep.deps === 1) {
          usage = "作者名";
        }
        if (v.deps === dep.deps) {
          usage = "作品名";
        }
        initialUsage[v.deps][dep.deps] = usage;
      });
    });

    setUsages(initialUsage);
    setIgnoreDeps(initialIgnore);
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
    const ignore = ignoreDeps();
    const targetPaths = paths().filter((v) => !ignore[v.dirDeps.length]);
    await commandNullWrapper(commandImportDirectory)({
      dirPathInfos: targetPaths,
      usages: usages(),
    });
  };

  const preview = () => {
    const maxDeps = targetMaxDeps();
    const usage = usages()[maxDeps];
    let artist = UNKNOWN_ARTIST_NAME;
    let title = "";
    const tags: Tag[] = [];
    const deps = eachDepsSample();
    for (let i = 1; i < maxDeps + 1; i++) {
      const u = usage[i];
      const name = deps[i - 1].name;
      if (u === "作品名") {
        title = name;
      }
      if (u === "作者名") {
        artist = name;
      }
      if (u === "タグ") {
        tags.push({ name: name, id: "", updatedAt: "" });
      }
    }
    return { title, artist, tags };
  };

  return {
    eachDepsSample,
    getUsage,
    setUsage,
    sampleSrc,
    dirDepsLengthKindOnlyDeps,
    confirm,
    preview,
    getIgnoreDeps,
    toggleIgnoreDeps,
  };
};

export default useDirUsage;
