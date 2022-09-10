import { Component, createResource, For } from "solid-js";
import {
  commandSelectWorkByArtist,
  commandUpdateArtistName,
} from "../../lib/commands";
import { commandWrapper } from "../../lib/toast";
import type { Artist as ArtistI } from "../../lib/types";
import ArtistWork from "./ArtistWork";
import Editor from "./Editor";
import HorizontalScroller from "./HorizontalScroller";

interface Props {
  artist: ArtistI;
  refetch: () => void;
  index?: number;
}
const Artist: Component<Props> = (props) => {
  const [works] = createResource(
    () => props.artist.id,
    commandWrapper(commandSelectWorkByArtist),
    {
      initialValue: [],
    }
  );
  const updateNameCommand = async (name: string) => {
    if (name === "") {
      throw Error("更新後の作者名が空文字です");
    }
    await commandWrapper(commandUpdateArtistName)({
      id: props.artist.id,
      name,
    });
  };

  return (
    <div class="w-full flex flex-col gap-2">
      <div class="flex w-artist-name">
        <Editor
          initialText={() => props.artist.name}
          command={updateNameCommand}
          refetch={props.refetch}
          inputClass="font-bold text-lg"
        />
      </div>
      <div class="">
        <HorizontalScroller
          isGradientFader={true}
          scrollStep={300}
          iconSize="md"
        >
          <div class="flex gap-4 p-4">
            <For each={works()}>
              {(work, i) => (
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
      </div>
    </div>
  );
};

export default Artist;
