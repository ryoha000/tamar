import { Component, createSignal } from "solid-js";
import MenuDialogWrapper, {
  MenuDialogDeleteIconButton,
  MenuDialogIconButton,
} from "../UI/MenuDialogWrapper";
import { RiDocumentFolderZipLine } from "solid-icons/ri";
import { TbDatabaseOff } from "solid-icons/tb";
import { RiEditorNodeTree } from "solid-icons/ri";
import FolderImportDialog from "./FolderImportDialog";
import { dialog } from "@tauri-apps/api";
import FileImportDialog from "./FileImportDialog";

interface Props {
  isOpen: boolean;
  close: () => void;
}

const MenuDialog: Component<Props> = (props) => {
  const [isOpenFolderDialog, setIsOpenFolderDialog] = createSignal(false);
  const [directory, setDirectory] = createSignal("");
  const openFolderDialog = async () => {
    const dir = await dialog.open({ directory: true });
    if (dir && !Array.isArray(dir)) {
      setDirectory(dir);
      setIsOpenFolderDialog(true);
      props.close();
    }
  };

  const [isOpenFileDialog, setIsOpenFileDialog] = createSignal(false);
  const [file, setFile] = createSignal<string[]>([]);
  const openFileDialog = async () => {
    const f = await dialog.open({
      multiple: true,
      filters: [{ name: "zip", extensions: ["zip", "ZIP"] }],
    });
    if (f) {
      if (Array.isArray(f)) {
        setFile(f);
      } else {
        setFile([f]);
      }
      setIsOpenFileDialog(true);
      props.close();
    }
  };

  return (
    <>
      <MenuDialogWrapper isOpen={props.isOpen} close={props.close}>
        <MenuDialogIconButton
          label="フォルダからインポート"
          icon={RiEditorNodeTree}
          click={openFolderDialog}
        />
        <MenuDialogIconButton
          label="ファイルからインポート"
          icon={RiDocumentFolderZipLine}
          click={openFileDialog}
        />
        <MenuDialogDeleteIconButton
          label="全ての登録作品を消す"
          icon={TbDatabaseOff}
          click={() => {}}
        />
      </MenuDialogWrapper>
      <FolderImportDialog
        isOpen={isOpenFolderDialog()}
        close={() => setIsOpenFolderDialog(false)}
        dir={directory()}
      />
      <FileImportDialog
        isOpen={isOpenFileDialog()}
        close={() => setIsOpenFileDialog(false)}
        filePaths={file()}
      />
    </>
  );
};

export default MenuDialog;
