import { useNavigate } from "@solidjs/router";
import { AiOutlineSearch } from "solid-icons/ai";
import { Component, createResource, For, Show } from "solid-js";
import {
  commandSelectWorkByArtist,
  commandUpdateArtistName,
} from "../../lib/commands";
import { commandArrayWrapper, commandNullWrapper } from "../../lib/toast";
import type { Artist as ArtistI } from "../../lib/types";
import ArtistWork from "./ArtistWork";
import Editor from "./Editor";
import HorizontalScroller from "./HorizontalScroller";
import TooltipWrapper from "./TooltipWrapper";

interface Props {
  artist: ArtistI;
  refetch: () => void;
  index?: number;
  displayMode?: "carousel" | "grid";
}
const Artist: Component<Props> = (props) => {
  const [works] = createResource(
    () => props.artist.id,
    commandArrayWrapper(commandSelectWorkByArtist),
    {
      initialValue: [],
    }
  );
  const updateNameCommand = async (name: string) => {
    if (name === "") {
      throw Error("更新後の作者名が空文字です");
    }
    await commandNullWrapper(commandUpdateArtistName)({
      id: props.artist.id,
      name,
    });
  };
  const navigator = useNavigate();
  const artistLink = () => `/artist/${props.artist.id}`;
  const goArtistPage = () => {
    navigator(artistLink());
  };
  const isGrid = () => props.displayMode === "grid";

  return (
    <div class="w-full flex flex-col gap-2">
      <div class="flex items-center gap-2 w-artist-name">
        <Editor
          initialText={() => props.artist.name}
          command={updateNameCommand}
          refetch={props.refetch}
          inputClass="font-bold text-lg"
          beforeEditButton={
            <Show when={!isGrid()}>
              <TooltipWrapper label="この作者で絞り込む">
                <button
                  type="button"
                  class="opacity-50 hover:opacity-80 hover:bg-secondary rounded transition-all p-1"
                  onclick={goArtistPage}
                  aria-label={`${props.artist.name}で絞り込む`}
                >
                  <AiOutlineSearch size="1.2rem" />
                </button>
              </TooltipWrapper>
            </Show>
          }
        />
      </div>
      <Show
        when={isGrid()}
        fallback={
          <HorizontalScroller
            isGradientFader={true}
            scrollStep={300}
            iconSize="md"
          >
            <div class="flex gap-4 p-4">
              <For each={works()}>
                {(work) => (
                  <div
                    class="w-44 h-44 p-2 flex-shrink-0"
                    style="content-visibility: auto;contain-intrinsic-size: 11rem;"
                  >
                    <ArtistWork work={work} index={props.index} />
                  </div>
                )}
              </For>
            </div>
          </HorizontalScroller>
        }
      >
        <div
          class="grid gap-4 p-4 w-full justify-center sm:justify-start"
          style="grid-template-columns: repeat(auto-fill, minmax(min(10rem, 100%), 10rem));"
        >
          <For each={works()}>
            {(work) => (
              <div
                class="w-40 h-40"
                style="content-visibility: auto;contain-intrinsic-size: 10rem;"
              >
                <ArtistWork work={work} />
              </div>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
};

export default Artist;
