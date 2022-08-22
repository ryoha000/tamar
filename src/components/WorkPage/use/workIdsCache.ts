import { useParams } from "@solidjs/router";
import { createSignal } from "solid-js";
import {
  commandSearchAroundTitleWork,
  commandSearchAroundUpdatedAtWork,
} from "../../../lib/commands";

interface AroundWorkRequest {
  limit: number;
  isBefore: boolean;
  col: "updated_at" | "title";
  currentWorkId: string;
  value: string;
}

const useWorkIdsCache = () => {
  const [loading, setLoading] = createSignal(false);
  const [workIds, setWorkIds] = createSignal<string[]>([]); // 並び順は isSortDesc に関係なく DESC
  const fetchWorkIds = async (req: AroundWorkRequest) => {
    if (loading()) {
      return;
    }
    setLoading(true);
    let res: string[];
    switch (req.col) {
      case "title":
        res = await commandSearchAroundTitleWork({ ...req, title: req.value });
        break;
      case "updated_at":
        res = await commandSearchAroundUpdatedAtWork({
          ...req,
          updated_at: req.value,
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
      if (req.isBefore) {
        newWorkIds = [..._prev, ...res];
      } else {
        newWorkIds = [...res, ..._prev];
      }
      return newWorkIds;
    });
    setLoading(false);
  };
  return { workIds, fetchWorkIds, loading };
};

export default useWorkIdsCache;
