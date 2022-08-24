import { useNavigate, useParams } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor, createSignal } from "solid-js";
import type { SortColumnKind, Work } from "../../../lib/types";
import useWorkIdsCache from "./workIdsCache";

const LIMIT = 20;
const usePage = (
  work: Accessor<Work | null>,
  workPageMap: Map<string, number>,
  isSortDesc: Accessor<boolean>,
  sortCol: Accessor<SortColumnKind>
) => {
  const params = useParams();
  const navigator = useNavigate();

  const page = () => {
    const page = +params["page"];
    if (isNaN(page)) {
      throw Error("page param is NaN");
    }
    return page;
  };

  const originalImageSrcArray = () => {
    const _work = work();
    if (!_work) {
      return [];
    }
    const sortedPaths = [..._work.paths];
    sortedPaths.sort(); // TODO: ソートがこれでいいのか考える
    return sortedPaths;
  };

  const imageSrcArray = () => {
    return originalImageSrcArray().map((v) => convertFileSrc(v));
  };

  const imageSrc = () => {
    if (page() < 0 || page() >= imageSrcArray().length) {
      return "";
    }
    return imageSrcArray()[page()];
  };

  const originalImageSrc = () => {
    if (page() < 0 || page() >= originalImageSrcArray().length) {
      return "";
    }
    return originalImageSrcArray()[page()];
  };

  const { workIds, fetchWorkIds, loading } = useWorkIdsCache();

  const setWorkPage = (nextPage: number) => {
    const workId = work()?.id;
    if (workId) {
      workPageMap.set(workId, nextPage);
    }
  };
  const prev = () => {
    const nextPage = page() - 1;
    if (nextPage < 0) {
      return;
    }
    navigator(`../${nextPage}`);
    setWorkPage(nextPage);
  };
  const next = () => {
    const nextPage = page() + 1;
    if (nextPage >= imageSrcArray().length) {
      return;
    }
    navigator(`../${nextPage}`);
    setWorkPage(nextPage);
  };
  // 作品間の遷移
  const navigateBetweenWork = async (step: number) => {
    if (loading()) {
      return;
    }
    const _work = work();
    const workId = _work?.id;
    if (!workId) {
      return;
    }

    const currentIndex = workIds().findIndex((v) => v === workId);
    const nextIndex = currentIndex + step;
    if (currentIndex === -1 || nextIndex < 0 || nextIndex >= workIds().length) {
      const value = sortCol() === "title" ? _work.title : _work.updatedAt;
      // TODO: artist filter
      await fetchWorkIds({
        currentWorkId: workId,
        isBefore: step > 0,
        col: sortCol(),
        limit: 20,
        value,
      });

      const currentIndex = workIds().findIndex((v) => v === workId);
      const nextIndex = currentIndex + step;
      if (
        currentIndex === -1 ||
        nextIndex < 0 ||
        nextIndex >= workIds().length
      ) {
        return; // fetch しても遷移先がない時
      }
    }
    const nextId = workIds()[nextIndex];
    navigator(`/work/${nextId}/${workPageMap.get(nextId) ?? 0}`, {
      resolve: false,
      replace: true,
    });
  };
  const up = async () => {
    await navigateBetweenWork(isSortDesc() ? -1 : 1);
  };
  const down = async () => {
    await navigateBetweenWork(isSortDesc() ? 1 : -1);
  };

  const keyDown = (e: KeyboardEvent) => {
    if (e.key === "ArrowRight") {
      next();
    }
    if (e.key === "ArrowLeft") {
      prev();
    }
    if (e.key === "ArrowUp") {
      up();
    }
    if (e.key === "ArrowDown") {
      down();
    }
    console.log(e.key);
  };

  const INITIAL_WHEEL_STATE = { x: 0, y: 0 };
  const [wheelState, setWheelState] = createSignal(INITIAL_WHEEL_STATE);
  const [wheelTimer, setWheelTimer] = createSignal(0);
  const wheel = (e: WheelEvent) => {
    const oldState = wheelState();
    const state = { x: oldState.x + e.deltaX, y: oldState.y + e.deltaY };
    setWheelState(state);
    clearTimeout(wheelTimer());
    setWheelTimer(
      setTimeout(() => {
        // 遷移
        const state = wheelState();

        if (Math.abs(state.x) > Math.abs(state.y)) {
          // 水平スクロールのとき
          if (Math.abs(state.x) > 80) {
            // 閾値は適当
            if (state.x < 0) {
              prev();
            } else {
              next();
            }
          }
        } else {
          // 垂直スクロールのとき
          // TODO
        }
        console.log(state);
        setWheelState(INITIAL_WHEEL_STATE);
      }, 50)
    );
  };

  return {
    imageSrc,
    imageSrcArray,
    prev,
    next,
    keyDown,
    wheel,
    originalImageSrc,
  };
};

export default usePage;
