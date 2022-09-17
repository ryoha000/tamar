import { Accessor, createSignal } from "solid-js";
import {
  commandGetWork,
  commandSearchAroundTitleWork,
  commandSearchAroundUpdatedAtWork,
  commandSearchAroundViewTimeWork,
  commandSelectWorkByArtist,
} from "../../../lib/commands";
import { commandArrayWrapper, commandNullWrapper } from "../../../lib/toast";

interface AroundWorkRequest {
  limit: number;
  isBefore: boolean;
  col: "updated_at" | "title" | "view_time";
  currentWorkId: string;
  value: string;
}

const useWorkIdsCache = (isFilterArtist: Accessor<boolean>) => {
  const [loading, setLoading] = createSignal(false);
  const [workIds, setWorkIds] = createSignal<string[]>([]); // 並び順は isSortDesc に関係なく DESC

  const fetchWorkListWorkIds = async (req: AroundWorkRequest) => {
    if (loading()) {
      return;
    }
    setLoading(true);
    let res: string[];
    switch (req.col) {
      case "title":
        res = await commandArrayWrapper(commandSearchAroundTitleWork)({
          ...req,
          title: req.value,
        });
        break;
      case "updated_at":
        res = await commandArrayWrapper(commandSearchAroundUpdatedAtWork)({
          ...req,
          updated_at: req.value,
        });
        break;
      case "view_time":
        res = await commandArrayWrapper(commandSearchAroundViewTimeWork)({
          ...req,
          workId: req.currentWorkId,
        });
        break;
      default:
        const exhaustedType: never = req.col;
        throw Error("unknown fetchWorks req.col");
    }

    setWorkIds((prev) => {
      let newWorkIds: string[];
      const _prev = [...prev];
      if (prev.length === 0) {
        _prev.push(req.currentWorkId);
      }
      // isBefore が true だと DESC, false だと ASC が帰ってくる
      if (req.isBefore) {
        newWorkIds = [..._prev, ...res];
      } else {
        newWorkIds = [...res.reverse(), ..._prev];
      }
      return newWorkIds;
    });
    setLoading(false);
  };

  const fetchArtistListWorkIds = async (req: AroundWorkRequest) => {
    if (loading()) {
      return;
    }
    setLoading(true);
    const work = await commandNullWrapper(commandGetWork)(req.currentWorkId);
    if (work) {
      const works = await commandArrayWrapper(commandSelectWorkByArtist)(
        work.artist.id
      );

      setWorkIds(works.map((v) => v.id));
    }
    setLoading(false);
  };

  const fetchWorkIds = async (req: AroundWorkRequest) => {
    if (isFilterArtist()) {
      await fetchArtistListWorkIds(req);
    } else {
      await fetchWorkListWorkIds(req);
    }
  };
  return { workIds, fetchWorkIds, loading };
};

export default useWorkIdsCache;
