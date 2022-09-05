import {
  Component,
  createEffect,
  createSignal,
  For,
  onMount,
  Show,
} from "solid-js";
import { listenImportDirProgress } from "../../lib/events";
import Dialog from "../UI/Dialog";
import DropDownMenu from "../UI/DropDownMenu";
import Loading from "../UI/Loading";
import { MenuDialogSection } from "../UI/MenuDialogWrapper";
import Tag from "../UI/Tag";
import FileImportEachDeps from "./FileImportEachDeps";
import useDirUsage, { DepsUsageKind } from "./use/dirUsage";
import useExplorDir from "./use/exploreDir";

interface Props {
  isOpen: boolean;
  dir: string;
  close: () => void;
}

const FolderImportDialog: Component<Props> = (props) => {
  const [selectedDirDeps, setSelectedDirDeps] = createSignal("");
  createEffect(() => {
    const options = dirDepsLengthKindOnlyDeps();
    if (options.length > 0) {
      setSelectedDirDeps(options[0]);
    }
  });
  const selectedDirDepsNumber = () => +selectedDirDeps();

  const { paths } = useExplorDir(props);

  const {
    eachDepsSample,
    getUsage,
    setUsage,
    sampleSrc,
    dirDepsLengthKindOnlyDeps,
    confirm,
    preview,
  } = useDirUsage(paths, selectedDirDepsNumber);

  const [loading, setLoading] = createSignal(false);
  const [processed, setProcessed] = createSignal(0);
  const loadingOverlayClick = (e: MouseEvent) => {
    e.stopPropagation();
  };

  const clickConfirm = async () => {
    setLoading(true);
    await confirm();
    setLoading(false);
    close();
  };

  onMount(async () => {
    await listenImportDirProgress(setProcessed);
  });

  return (
    <>
      <Dialog isOpen={props.isOpen} close={props.close}>
        <div class="flex flex-col gap-2">
          <div class="text-xl font-bold">フォルダからインポート</div>
          <div class="flex flex-col gap-4 pl-4">
            <MenuDialogSection label="選択したフォルダ">
              <code class="text-sm">{props.dir}</code>
            </MenuDialogSection>
            <div class="flex flex-col">
              <div class="flex items-center gap-2">
                <DropDownMenu
                  options={dirDepsLengthKindOnlyDeps()}
                  selectedOption={selectedDirDeps()}
                  onChange={(opt) => setSelectedDirDeps(opt)}
                  width="3rem"
                />
                <div class="text-xl font-bold">階層ある場合の設定</div>
              </div>
              <div class="flex flex-col gap-2  pl-4">
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
            <MenuDialogSection label="プレビュー">
              <div class="flex gap-4">
                <img class="h-40 object-contain" src={sampleSrc()} />
                <div class="grid grid-cols-2 gap-x-4 grid-rows-import-preview">
                  <div>作品名</div>
                  <div>{preview().title}</div>
                  <div>作者名</div>
                  <div>{preview().artist}</div>
                  <div>タグ</div>
                  <div class="flex items-center gap-2 flex-wrap">
                    <For each={preview().tags}>
                      {(tag, i) => <Tag tag={tag} />}
                    </For>
                  </div>
                </div>
              </div>
            </MenuDialogSection>
            <div class="flex justify-center">
              <button
                onclick={clickConfirm}
                class="px-4 py-2 bg-primary hover:bg-secondary transition-all rounded text-white font-bold"
              >
                確定
              </button>
            </div>
          </div>
        </div>
      </Dialog>
      <Show when={loading()}>
        <div
          class="absolute left-0 top-0 w-full h-full bg-text-opacity-50 z-dialog-loading flex items-center justify-center"
          onclick={loadingOverlayClick}
        >
          <div class="flex flex-col items-center justify-center bg-white rounded p-8">
            <div>
              processing {processed()}/{paths().length}
            </div>
            <Loading />
          </div>
        </div>
      </Show>
    </>
  );
};

export default FolderImportDialog;
