import { IconTypes } from "solid-icons";
import { AiOutlineFolderOpen } from "solid-icons/ai";
import { BsFileEarmarkX, BsFolderX } from "solid-icons/bs";
import { Component, ParentComponent } from "solid-js";
import {
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
  close: () => void;
  refetch: () => void;
}

const MenuDialog: Component<Props> = (props) => {
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

  return (
    <DialogBase
      isOpen={props.isOpen}
      close={props.close}
      withCurtain={true}
      align="left"
    >
      <div class="flex gap-2 flex-col">
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
        />
        <MenuDialogIconButton label="ファイルを消す" icon={BsFileEarmarkX} />
        <MenuDialogIconButton label="作品を消す" icon={BsFolderX} />
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
  icon: IconTypes;
}> = (props) => {
  return (
    <button class="rounded px-4 py-2 hover:bg-secondary transition-all">
      <div class="flex items-center gap-2">
        {props.icon({ size: "1.5rem" })}
        <div>{props.label}</div>
      </div>
    </button>
  );
};

export default MenuDialog;
