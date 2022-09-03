import { IconTypes } from "solid-icons";
import { ParentComponent } from "solid-js";
import DialogBase from "./DialogBase";

interface Props {
  isOpen: boolean;
  close: () => void;
}

const MenuDialogWrapper: ParentComponent<Props> = (props) => {
  return (
    <DialogBase
      isOpen={props.isOpen}
      close={props.close}
      withCurtain={true}
      align="left"
    >
      <div class="flex gap-2 flex-col h-full">{props.children}</div>
    </DialogBase>
  );
};

export const MenuDialogSection: ParentComponent<{ label: string }> = (
  props
) => {
  return (
    <div class="flex flex-col gap-2">
      <div class="text-lg font-bold">{props.label}</div>
      <div class="pl-4">{props.children}</div>
    </div>
  );
};

export const MenuDialogIconButton: ParentComponent<{
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

export const MenuDialogDeleteIconButton: ParentComponent<{
  label: string;
  click: () => void;
  icon: IconTypes;
}> = (props) => {
  return (
    <MenuDialogIconButton
      buttonClass="bg-error mt-auto text-slate-50 align-bottom hover:text-text" // TODO: 色大丈夫？
      label={props.label}
      icon={props.icon}
      click={props.click}
    />
  );
};

export default MenuDialogWrapper;
