import { fs, path } from "@tauri-apps/api";
import { createEffect, createSignal } from "solid-js";

export interface DirPathInfo {
  path: string;
  name: string;
  dirDeps: DirDeps[];
}

export interface DirDeps {
  deps: number;
  name: string;
}

const getChildren = async (entry: fs.FileEntry) => {
  try {
    return await fs.readDir(entry.path);
  } catch {
    return [];
  }
};

const isFile = async (entry: fs.FileEntry) => {
  try {
    const ext = await path.extname(entry.path);
    return ext !== "";
  } catch {
    return false;
  }
};

const getFileName = async (p: string) => {
  const parent = await path.dirname(p);
  return p.replace(parent, "").replace(path.sep, "");
};

// entry がファイルをもつディレクトリの場合callbackを実行する
const exploreDir = async (
  entry: fs.FileEntry,
  callback: (e: fs.FileEntry, ds: string[]) => void,
  dirs: string[]
) => {
  const children = await getChildren(entry);

  // entry がディレクトリのとき
  if (children.length > 0) {
    let hasFile = false;
    const promises: Promise<void>[] = [];

    const newDirs = [...dirs];
    if (!entry.name) {
      console.warn("entry.name is null. entry: ", entry);
      return;
    }
    newDirs.push(entry.name);

    for (const child of children) {
      // entry がファイルをもつディレクトリの場合callbackを実行する
      if (await isFile(child)) {
        hasFile = true;
        continue;
      }
      promises.push(exploreDir(child, callback, newDirs));
    }

    if (hasFile) {
      callback(entry, newDirs);
    }
    await Promise.all(promises);
  }
  return;
};

const useExplorDir = (props: { dir: string }) => {
  const [loading, setLoading] = createSignal(false);
  const [paths, setPaths] = createSignal<DirPathInfo[]>([]);

  createEffect(async () => {
    if (props.dir === "") {
      return;
    }
    setPaths([]);
    setLoading(true);
    const _paths: DirPathInfo[] = [];

    const callback = (entry: fs.FileEntry, dirs: string[]) => {
      const deps: DirDeps[] = [];
      for (let i = 0; i < dirs.length; i++) {
        deps.push({ deps: i + 1, name: dirs[i] });
      }
      if (!entry.name) {
        console.warn("entry.name is null. entry: ", entry);
        return;
      }
      _paths.push({ path: entry.path, name: entry.name, dirDeps: deps });
    };

    try {
      await exploreDir(
        { path: props.dir, name: await getFileName(props.dir) },
        callback,
        []
      );
      console.log({ _paths });
      setPaths(_paths);
    } catch {}
    setLoading(false);
  });

  const getEachDepsSample = () => {
    if (paths().length === 0) {
      return [];
    }
    let maxIndex = 0;
    for (let i = 0; i < paths().length; i++) {
      if (paths()[maxIndex].dirDeps.length < paths()[i].dirDeps.length) {
        paths()[maxIndex].dirDeps.length = paths()[i].dirDeps.length;
      }
    }

    return paths()[maxIndex].dirDeps;
  };

  return { loading, paths, getEachDepsSample };
};

export default useExplorDir;
