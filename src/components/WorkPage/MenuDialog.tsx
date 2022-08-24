import { AiOutlineFolderOpen } from "solid-icons/ai";
import { Component, ParentComponent } from "solid-js";
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
            command={async () => {}}
            refetch={props.refetch}
            inputClass="text-lg"
          />
        </MenuDialogSection>

        <MenuDialogSection label="作者">
          <Editor
            initialText={props.work.artist.name}
            command={async () => {}}
            fetchSuggests={async () => []}
            refetch={props.refetch}
          />
        </MenuDialogSection>

        <MenuDialogSection label="タグ">
          <MenuDialogTagList tags={props.work.tags} />
        </MenuDialogSection>

        <button>
          <AiOutlineFolderOpen size="1.5rem" />
        </button>
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

export default MenuDialog;
