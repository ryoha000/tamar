import { Link } from "@solidjs/router";
import { Component, For, Show } from "solid-js";
import { Work } from "../../lib/types";
import Dialog from "../UI/Dialog";
import Tag from "../UI/Tag";

interface Props {
  work: Work | null;
  imageSrcArray: string[];
  isOpen: boolean;
  close: () => void;
}

const ImageListDialog: Component<Props> = (props) => {
  return (
    <Dialog isOpen={props.isOpen} close={props.close}>
      <Show when={props.work}>
        <div class="flex flex-col">
          <div class="grid grid-cols-2 gap-4">
            <img
              class="h-48 object-contain ml-auto"
              src={props.imageSrcArray.length ? props.imageSrcArray[0] : ""}
            />
            <div class="flex flex-col gap-2 p-2">
              <div class="text-lg font-bold">{props.work!.title}</div>
              <div class="cursor-pointer font-bold">
                {props.work!.artist.name}
              </div>
              <div class="flex flex-wrap gap-2">
                <For each={props.work!.tags}>
                  {(tag, i) => <Tag tag={tag} />}
                </For>
                <For each={props.work!.tags}>
                  {(tag, i) => <Tag tag={tag} />}
                </For>
                <For each={props.work!.tags}>
                  {(tag, i) => <Tag tag={tag} />}
                </For>
                <For each={props.work!.tags}>
                  {(tag, i) => <Tag tag={tag} />}
                </For>
                <For each={props.work!.tags}>
                  {(tag, i) => <Tag tag={tag} />}
                </For>
              </div>
            </div>
          </div>
          <div class="grid grid-cols-image-list p-4 gap-2">
            <For each={props.imageSrcArray}>
              {(src, i) => (
                <Link href={`../${i()}`} class="hover:scale-105 transition-all">
                  <img class="w-full h-full object-contain" src={src} />
                </Link>
              )}
            </For>
          </div>
        </div>
      </Show>
    </Dialog>
  );
};

export default ImageListDialog;
