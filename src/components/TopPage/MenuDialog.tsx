import { Component, createSignal } from "solid-js";
import MenuDialogWrapper, {
  MenuDialogDeleteIconButton,
  MenuDialogIconButton,
} from "../UI/MenuDialogWrapper";
import { RiDocumentFolderLine, RiDocumentFolderZipLine } from "solid-icons/ri";
import { TbDatabaseOff } from "solid-icons/tb";
import { RiEditorNodeTree } from "solid-icons/ri";
import FolderImportDialog from "./FolderImportDialog";
import { dialog } from "@tauri-apps/api";
import FileImportDialog from "./FileImportDialog";
import { commandDeleteAllData } from "../../lib/commands";
import { confirm } from "@tauri-apps/api/dialog";

interface Props {
  isOpen: boolean;
  close: () => void;
  refetch: () => void;
}

const MenuDialog: Component<Props> = (props) => {
  const [isOpenFolderStructureDialog, setIsOpenFolderStructureDialog] =
    createSignal(false);
  const [directory, setDirectory] = createSignal("");
  const openFolderStructureDialog = async () => {
    const dir = await dialog.open({ directory: true });
    if (dir && !Array.isArray(dir)) {
      setDirectory(dir);
      setIsOpenFolderStructureDialog(true);
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

  const [isOpenFolderDialog, setIsOpenFolderDialog] = createSignal(false);
  const [folder, setFolder] = createSignal<string[]>([]);
  const openFolderDialog = async () => {
    const f = await dialog.open({
      multiple: true,
      directory: true,
    });
    if (f) {
      if (Array.isArray(f)) {
        setFolder(f);
      } else {
        setFolder([f]);
      }
      setIsOpenFolderDialog(true);
      props.close();
    }
  };

  const deleteAllData = async () => {
    const res = await confirm(
      "ほんとうにすべてのデータを削除しますか？(この操作は取り消せません)",
      { type: "warning" }
    );
    if (res) {
      await commandDeleteAllData();
      location.href = "/";
    }
  };

  return (
    <>
      <MenuDialogWrapper isOpen={props.isOpen} close={props.close}>
        <MenuDialogIconButton
          label="フォルダ構造からインポート"
          icon={RiEditorNodeTree}
          click={openFolderStructureDialog}
        />
        <MenuDialogIconButton
          label="ファイルからインポート"
          icon={RiDocumentFolderZipLine}
          click={openFileDialog}
        />
        <MenuDialogIconButton
          label="フォルダーからインポート"
          icon={RiDocumentFolderLine}
          click={openFolderDialog}
        />
        <MenuDialogDeleteIconButton
          label="全ての登録作品を消す"
          icon={TbDatabaseOff}
          click={deleteAllData}
        />
      </MenuDialogWrapper>
      <FolderImportDialog
        isOpen={isOpenFolderStructureDialog()}
        close={() => setIsOpenFolderStructureDialog(false)}
        dir={directory()}
        refetch={props.refetch}
      />
      <FileImportDialog
        isOpen={isOpenFileDialog()}
        close={() => setIsOpenFileDialog(false)}
        filePaths={file()}
        refetch={props.refetch}
      />
      <FileImportDialog
        isOpen={isOpenFolderDialog()}
        close={() => setIsOpenFolderDialog(false)}
        filePaths={folder()}
        refetch={props.refetch}
      />
    </>
  );
};

export default MenuDialog;
