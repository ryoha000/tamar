import { Component, createResource, createSignal, For, Show } from "solid-js";
import {
  commandImportFile,
  commandSelectArtistByName,
} from "../../lib/commands";
import { UNKNOWN_ARTIST_NAME } from "../../lib/types";
import Dialog from "../UI/Dialog";
import { MenuDialogSection } from "../UI/MenuDialogWrapper";

interface Props {
  isOpen: boolean;
  filePaths: string[];
  close: () => void;
}

const FileImportDialog: Component<Props> = (props) => {
  const [artist, setArtist] = createSignal(UNKNOWN_ARTIST_NAME);

  const artistInput = (
    e: Event & { currentTarget: HTMLInputElement; target: Element }
  ) => {
    if (e.target instanceof HTMLInputElement) {
      setArtist(e.target.value);
    }
  };

  const [artistOptions] = createResource(artist, commandSelectArtistByName, {
    initialValue: [],
  });
  const options = () => artistOptions().map((v) => v.name);

  const [isFocusInput, setIsFocusInput] = createSignal(false);

  const confirm = async () => {
    await commandImportFile({
      artistName: artist(),
      filePaths: props.filePaths,
    });
    props.close();
    // TODO: refetch
  };

  return (
    <Show when={props.filePaths.length}>
      <Dialog isOpen={props.isOpen} close={props.close}>
        <div class="flex flex-col gap-2">
          <div class="text-xl font-bold">ファイルからインポート</div>
          <div class="flex flex-col gap-4 pl-4">
            <MenuDialogSection label="選択したファイル">
              <code class="text-sm">
                {props.filePaths[0]}
                {props.filePaths.length === 1 ? "" : " ..."}
              </code>
            </MenuDialogSection>
            <MenuDialogSection label="作者名">
              <div class="relative w-artist-name">
                <input
                  list="file-import-dialog-artist w-full"
                  value={artist()}
                  oninput={artistInput}
                  onfocus={() => setIsFocusInput(true)}
                  onfocusout={() => setIsFocusInput(false)}
                ></input>
                <datalist id="file-import-dialog-artist">
                  <For each={options()}>
                    {(option, i) => <option>{option}</option>}
                  </For>
                </datalist>
                <div
                  classList={{
                    "scale-0": !isFocusInput(),
                    "scale-100": isFocusInput(),
                  }}
                  class="absolute bottom-0 left-0 h-0.5 w-full bg-secondary transition-all"
                ></div>
              </div>
            </MenuDialogSection>
            <div class="flex justify-center">
              <button
                onclick={confirm}
                class="px-4 py-2 bg-primary hover:bg-secondary transition-all rounded text-white font-bold"
              >
                確定
              </button>
            </div>
          </div>
        </div>
      </Dialog>
    </Show>
  );
};

export default FileImportDialog;
