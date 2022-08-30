import { useParams } from "@solidjs/router";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { Accessor } from "solid-js";
import type {  Work } from "../../../lib/types";

const useImage = (
  work: Accessor<Work | null>,
) => {
  const params = useParams();

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

  return {
    imageSrc,
    imageSrcArray,
    originalImageSrc,
  };
};

export default useImage;
