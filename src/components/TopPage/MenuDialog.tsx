import { useNavigate } from "@solidjs/router";
import { path, shell } from "@tauri-apps/api";
import { confirm } from "@tauri-apps/api/dialog";
import { AiOutlineFolderOpen, AiOutlineRotateRight } from "solid-icons/ai";
import { BsFileEarmarkX, BsFolderX } from "solid-icons/bs";
import { Component } from "solid-js";
import MenuDialogWrapper, {
  MenuDialogDeleteIconButton,
  MenuDialogIconButton,
} from "../UI/MenuDialogWrapper";

interface Props {
  isOpen: boolean;
  close: () => void;
}

const MenuDialog: Component<Props> = (props) => {
  return (
    <MenuDialogWrapper isOpen={props.isOpen} close={props.close}>
      <MenuDialogIconButton
        label="フォルダを開く"
        icon={AiOutlineFolderOpen}
        click={() => {}}
      />
      <MenuDialogIconButton
        label="このファイルを回転"
        icon={AiOutlineRotateRight}
        click={() => {}}
      />
      <MenuDialogIconButton
        label="ファイルを消す"
        icon={BsFileEarmarkX}
        click={() => {}}
      />
      <MenuDialogDeleteIconButton
        label="作品を消す"
        icon={BsFolderX}
        click={() => {}}
      />
    </MenuDialogWrapper>
  );
};

export default MenuDialog;
