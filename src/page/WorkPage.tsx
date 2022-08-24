import { useParams } from "@solidjs/router";
import {
  Component,
  createResource,
  createSignal,
  onMount,
  Show,
} from "solid-js";
import Header from "../components/WorkPage/Header";
import ImageListDialog from "../components/WorkPage/ImageListDialog";
import {
  NextOverlay,
  PrevOverlay,
} from "../components/WorkPage/NavigationOverlay";
import usePage from "../components/WorkPage/use/page";
import MenuDialog from "../components/WorkPage/MenuDialog";
import { commandGetWork } from "../lib/commands";
import { useStore } from "../lib/store";

const WorkPage: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }
  const params = useParams();

  onMount(() => {
    console.log("onMount WorkPage");
    // TODO: 閲覧履歴を insert する
  });

  const { workPageMap, isSortDesc, sortCol } = store;
  const [work, { refetch }] = createResource(
    () => params["id"],
    commandGetWork,
    {
      initialValue: null,
    }
  );

  const { imageSrc, imageSrcArray, next, prev, keyDown, wheel } = usePage(
    work,
    workPageMap,
    isSortDesc,
    sortCol
  );

  const [isListOpen, setIsListOpen] = createSignal(false);

  const [isOpenMenuDialog, setIsOpenMenuDialog] = createSignal(false);

  return (
    <div class="flex" onkeydown={keyDown}>
      <Header
        openListDialog={() => setIsListOpen(true)}
        openMenuDialog={() => setIsOpenMenuDialog(true)}
        workTitle={work()?.title ?? ""}
      />
      <Show when={work()}>
        <img
          src={imageSrc()}
          tabIndex={-1}
          // @ts-ignore
          autofocus
          class="w-screen h-screen object-contain"
          onwheel={wheel}
          onKeyDown={keyDown}
        ></img>
        <NextOverlay navigate={next} />
        <PrevOverlay navigate={prev} />
        <ImageListDialog
          work={work()}
          imageSrcArray={imageSrcArray()}
          isOpen={isListOpen()}
          close={() => setIsListOpen(false)}
        />
        <MenuDialog
          work={work()!}
          isOpen={isOpenMenuDialog()}
          close={() => setIsOpenMenuDialog(false)}
          refetch={refetch}
        />
      </Show>
    </div>
  );
};

export default WorkPage;
