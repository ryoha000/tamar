import { Component, createSignal, For } from "solid-js";
import Dialog from "../UI/Dialog";
import DropDownMenu from "../UI/DropDownMenu";
import FileImportEachDeps from "./FileImportEachDeps";
import useDirUsage, { DepsUsageKind } from "./use/dirUsage";
import useExplorDir from "./use/exploreDir";

interface Props {
  isOpen: boolean;
  dir: string;
  close: () => void;
}

const FileImportDialog: Component<Props> = (props) => {
  const { paths } = useExplorDir(props);
  const { eachDepsSample, getUsage, setUsage, sampleSrc } = useDirUsage(paths);

  return (
    <Dialog isOpen={props.isOpen} close={props.close}>
      <div class="flex flex-col gap-2">
        <div class="text-xl font-bold">フォルダからインポート</div>
        <div class="flex flex-col gap-4 pl-4">
          <div>
            <div>選択したフォルダ</div>
            <code class="text-sm">{props.dir}</code>
          </div>
          <div class="flex flex-col">
            <div>階層による設定</div>
            <div class="flex flex-col gap-2">
              <For each={eachDepsSample()}>
                {(deps, i) => (
                  <FileImportEachDeps
                    deps={deps}
                    selectedUsage={getUsage(deps.deps)}
                    onChange={(usage) =>
                      setUsage(deps.deps, usage as DepsUsageKind)
                    }
                  />
                )}
              </For>
            </div>
          </div>
          <div class="flex flex-col gap-2">
            <div>プレビュー</div>
            <div class="flex gap-4">
              <img class="h-40 object-contain" src={sampleSrc()} />
              <div>
                <div>作品名: {"aa"}</div>
                <div>作者名: {"aa"}</div>
                <div>タグ: {"aa"}</div>
              </div>
            </div>
          </div>
          <div class="flex justify-center">
            <button class="px-4 py-2 bg-primary hover:bg-secondary transition-all rounded text-white font-bold">
              確定
            </button>
          </div>
        </div>
      </div>
    </Dialog>
  );
};

export default FileImportDialog;
