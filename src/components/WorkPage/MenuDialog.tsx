import { useNavigate } from "@solidjs/router";
import { path, shell } from "@tauri-apps/api";
import { confirm } from "@tauri-apps/api/dialog";
import { IconTypes } from "solid-icons";
import { AiOutlineFolderOpen, AiOutlineRotateRight } from "solid-icons/ai";
import { BsFileEarmarkX, BsFolderX } from "solid-icons/bs";
import { Component, ParentComponent } from "solid-js";
import {
  commandDeleteWork,
  commandDeleteWorkFile,
  commandRotateWorkFile,
  commandSelectArtistByName,
  commandUpdateWorkArtist,
  commandUpdateWorkTitle,
} from "../../lib/commands";
import { errorToast } from "../../lib/toast";
import { Work } from "../../lib/types";
import DialogBase from "../UI/DialogBase";
import Editor from "../UI/Editor";
import MenuDialogTagList from "./MenuDialogTagList";

interface Props {
  isOpen: boolean;
  work: Work;
  imageSrc: string;
  close: () => void;
  refetch: () => void;
  refetchImage: () => void;
}

const MenuDialog: Component<Props> = (props) => {
  const navigator = useNavigate();

  const titleCommand = async (title: string) => {
    if (title === "") {
      errorToast("変更後のタイトルが空文字です");
      return;
    }
    await commandUpdateWorkTitle(props.work.id, title);
    props.refetch();
  };

  const artistCommand = async (name: string) => {
    if (name === "") {
      errorToast("変更後の作者名が空文字です");
      return;
    }
    await commandUpdateWorkArtist(props.work.id, name);
    props.refetch();
  };

  const fetchArtistSuggest = async (text: string) => {
    return (await commandSelectArtistByName(text)).map((v) => v.name);
  };

  const openExplorer = async () => {
    const p = await path.dirname(props.imageSrc);
    shell.open(p);
    props.close();
  };

  const deleteWork = async () => {
    if (await confirm("本当にこの作品を削除しますか？")) {
      await commandDeleteWork(props.work.id);
      props.close();
      navigator("/");
    }
  };

  const rotate = async () => {
    try {
      // TODO: アホ重いから非同期でやって表示してるやつは transform で回転させる(refetchImage -> rotateImage)
      await commandRotateWorkFile(props.imageSrc);
      props.refetchImage();
    } catch (e) {
      errorToast(`画像回転に失敗しました。error: ${e}`);
      console.error(e);
    }
    props.close();
  };

  const deleteFile = async () => {
    await commandDeleteWorkFile(props.imageSrc);
    props.refetch(); // TODO: これでpage外にいくとめんどくさい
    props.close();
  };

  return (
    <DialogBase
      isOpen={props.isOpen}
      close={props.close}
      withCurtain={true}
      align="left"
    >
      <div class="flex gap-2 flex-col h-full">
        <MenuDialogSection label="タイトル">
          <Editor
            initialText={props.work.title}
            command={titleCommand}
            refetch={props.refetch}
            inputClass="text-lg"
          />
        </MenuDialogSection>

        <MenuDialogSection label="作者">
          <Editor
            initialText={props.work.artist.name}
            command={artistCommand}
            fetchSuggests={fetchArtistSuggest}
            refetch={props.refetch}
          />
        </MenuDialogSection>

        <MenuDialogSection label="タグ">
          <MenuDialogTagList
            workId={props.work.id}
            tags={props.work.tags}
            refetch={props.refetch}
          />
        </MenuDialogSection>

        <MenuDialogIconButton
          label="フォルダを開く"
          icon={AiOutlineFolderOpen}
          click={openExplorer}
        />
        <MenuDialogIconButton
          label="このファイルを回転"
          icon={AiOutlineRotateRight}
          click={rotate}
        />
        <MenuDialogIconButton
          label="ファイルを消す"
          icon={BsFileEarmarkX}
          click={deleteFile}
        />
        <MenuDialogIconButton
          buttonClass="bg-error mt-auto text-slate-50 align-bottom hover:text-text" // TODO: 色大丈夫？
          label="作品を消す"
          icon={BsFolderX}
          click={deleteWork}
        />
      </div>
    </DialogBase>
  );
};

const MenuDialogSection: ParentComponent<{ label: string }> = (props) => {
  return (
    <div class="flex flex-col gap-2">
      <div class="text-lg font-bold">{props.label}</div>
      <div class="pl-4">{props.children}</div>
    </div>
  );
};

const MenuDialogIconButton: ParentComponent<{
  label: string;
  buttonClass?: string;
  click: () => void;
  icon: IconTypes;
}> = (props) => {
  return (
    <button
      onclick={props.click}
      class={`rounded px-4 py-2 hover:bg-secondary transition-all ${
        props.buttonClass ?? ""
      }`}
    >
      <div class="flex items-center gap-2">
        {props.icon({ size: "1.5rem" })}
        <div>{props.label}</div>
      </div>
    </button>
  );
};

export default MenuDialog;
