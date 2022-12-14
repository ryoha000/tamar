import { useNavigate } from "@solidjs/router";
import { path, shell } from "@tauri-apps/api";
import { confirm } from "@tauri-apps/api/dialog";
import { AiOutlineFolderOpen, AiOutlineRotateRight } from "solid-icons/ai";
import { BsFileEarmarkX, BsFolderX } from "solid-icons/bs";
import { Component } from "solid-js";
import {
  commandDeleteWork,
  commandDeleteWorkFile,
  commandRotateWorkFile,
  commandSelectArtistByName,
  commandUpdateWorkArtist,
  commandUpdateWorkTitle,
} from "../../lib/commands";
import {
  commandArrayWrapper,
  commandNullWrapper,
  errorToast,
} from "../../lib/toast";
import { Work } from "../../lib/types";
import Editor from "../UI/Editor";
import MenuDialogWrapper, {
  MenuDialogDeleteIconButton,
  MenuDialogIconButton,
  MenuDialogSection,
} from "../UI/MenuDialogWrapper";
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
    await commandNullWrapper(commandUpdateWorkTitle)({
      id: props.work.id,
      title: title,
    });
    props.refetch();
  };

  const artistCommand = async (name: string) => {
    if (name === "") {
      errorToast("変更後の作者名が空文字です");
      return;
    }
    await commandNullWrapper(commandUpdateWorkArtist)({
      id: props.work.id,
      name,
    });
    props.refetch();
  };

  const fetchArtistSuggest = async (text: string) => {
    return (await commandArrayWrapper(commandSelectArtistByName)(text)).map(
      (v) => v.name
    );
  };

  const openExplorer = async () => {
    const p = await path.dirname(props.imageSrc);
    shell.open(p);
    props.close();
  };

  const deleteWork = async () => {
    if (await confirm("本当にこの作品を削除しますか？")) {
      await commandNullWrapper(commandDeleteWork)(props.work.id);
      props.close();
      navigator("/");
    }
  };

  const rotate = async () => {
    try {
      // TODO: アホ重いから非同期でやって表示してるやつは transform で回転させる(refetchImage -> rotateImage)
      await commandNullWrapper(commandRotateWorkFile)(props.imageSrc);
      props.refetchImage();
    } catch (e) {
      errorToast(`画像回転に失敗しました。error: ${e}`);
      console.error(e);
    }
    props.close();
  };

  const deleteFile = async () => {
    await commandNullWrapper(commandDeleteWorkFile)(props.imageSrc);
    props.refetch(); // TODO: これでpage外にいくとめんどくさい
    props.close();
  };

  return (
    <MenuDialogWrapper isOpen={props.isOpen} close={props.close}>
      <MenuDialogSection label="タイトル">
        <Editor
          initialText={() => props.work.title}
          command={titleCommand}
          refetch={props.refetch}
          inputClass="text-lg"
        />
      </MenuDialogSection>

      <MenuDialogSection label="作者">
        <Editor
          initialText={() => props.work.artist.name}
          command={artistCommand}
          fetchSuggests={fetchArtistSuggest}
          refetch={props.refetch}
          link={`/artist/${props.work.artist.id}`}
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
      <MenuDialogDeleteIconButton
        label="作品を消す"
        icon={BsFolderX}
        click={deleteWork}
      />
    </MenuDialogWrapper>
  );
};

export default MenuDialog;
