import { useNavigate, useParams } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor, createSignal } from "solid-js";
import { Work } from "../../../lib/types";

const usePage = (
  work: Accessor<Work | null>,
  workPageMap: Map<string, number>
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

  const imageSrcArray = () => {
    const _work = work();
    if (!_work) {
      return [];
    }
    const sortedPaths = [..._work.paths];
    sortedPaths.sort(); // TODO: ソートがこれでいいのか考える
    return sortedPaths.map((v) => convertFileSrc(v));
  };

  const imageSrc = () => {
    if (page() < 0 || page() >= imageSrcArray().length) {
      console.error(`page: ${page()}, image.len: ${imageSrcArray().length}`);
      return "";
    }
    return imageSrcArray()[page()];
  };

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

  const keyDown = (e: KeyboardEvent) => {
    if (e.key === "ArrowRight") {
      next();
    }
    if (e.key === "ArrowLeft") {
      prev();
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

  return { imageSrc, imageSrcArray, prev, next, keyDown, wheel };
};

export default usePage;
