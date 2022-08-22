import { useNavigate, useParams } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor } from "solid-js";
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

  return { imageSrc, prev, next, keyDown };
};

export default usePage;
