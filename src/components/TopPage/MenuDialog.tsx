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
          click={() => {}}
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
    </>
  );
};

export default MenuDialog;
