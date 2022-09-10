import { useParams } from "@solidjs/router";
import {
  Accessor,
  Component,
  createEffect,
  createResource,
  createSignal,
  onMount,
  Resource,
  Setter,
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
import { commandGetWork, commandViewWork } from "../lib/commands";
import { useStore } from "../lib/store";
import useImage from "../components/WorkPage/use/image";
import { Work } from "../lib/types";
import { commandWrapper } from "../lib/toast";

const WorkPage: Component = () => {
  const params = useParams();

  const [work, { refetch }] = createResource(
    () => params["id"],
    commandWrapper(commandGetWork),
    {
      initialValue: null,
    }
  );

  const [isListOpen, setIsListOpen] = createSignal(false);
  const [isOpenMenuDialog, setIsOpenMenuDialog] = createSignal(false);

  return (
    <div class="flex bg-background">
      <Header
        openListDialog={() => setIsListOpen(true)}
        openMenuDialog={() => setIsOpenMenuDialog(true)}
        workTitle={work()?.title ?? ""}
      />
      <Show when={work()}>
        <WorkPageContent
          work={work}
          refetch={refetch}
          isListOpen={isListOpen}
          setIsListOpen={setIsListOpen}
          isOpenMenuDialog={isOpenMenuDialog}
          setIsOpenMenuDialog={setIsOpenMenuDialog}
        />
      </Show>
    </div>
  );
};

interface ContentProps {
  work: Resource<Work | null>;
  refetch: () => void;
  isListOpen: Accessor<boolean>;
  setIsListOpen: Setter<boolean>;
  isOpenMenuDialog: Accessor<boolean>;
  setIsOpenMenuDialog: Setter<boolean>;
}

const WorkPageContent: Component<ContentProps> = (props) => {
  let imgEle: HTMLImageElement | undefined = undefined;
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  onMount(() => {
    if (imgEle) {
      imgEle.click();
      imgEle.focus();
    }
  });

  createEffect(async () => {
    const workId = props.work()?.id;
    if (workId) {
      await commandViewWork(workId);
    }
  });

  const { workPageMap, isSortDesc, sortCol } = store;

  const { imageSrc, imageSrcArray, originalImageSrc } = useImage(props.work);

  const { next, prev, keyDown, wheel } = usePage(
    props.work,
    workPageMap,
    store.isFilterArtist,
    isSortDesc,
    sortCol,
    imageSrcArray
  );

  const [imageCacheKey, setImageCacheKey] = createSignal("");
  const refreshImage = () => {
    setImageCacheKey(`${Math.random()}`);
  };

  return (
    <>
      <img
        src={`${imageSrc()}?${imageCacheKey()}`}
        tabIndex={-1}
        // @ts-ignore
        autofocus
        class="w-screen h-screen object-contain"
        onwheel={wheel}
        onKeyDown={keyDown}
        ref={imgEle}
      ></img>
      <NextOverlay navigate={next} />
      <PrevOverlay navigate={prev} />
      <ImageListDialog
        work={props.work()}
        imageSrcArray={imageSrcArray()}
        isOpen={props.isListOpen()}
        close={() => props.setIsListOpen(false)}
      />
      <MenuDialog
        work={props.work()!}
        imageSrc={originalImageSrc()}
        isOpen={props.isOpenMenuDialog()}
        close={() => props.setIsOpenMenuDialog(false)}
        refetch={props.refetch}
        refetchImage={refreshImage}
      />
    </>
  );
};

export default WorkPage;
