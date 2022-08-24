import { Component, createResource, For } from "solid-js";
import { commandSelectWorkByArtist } from "../../lib/commands";
import type { Artist as ArtistI } from "../../lib/types";
import ArtistWork from "./ArtistWork";
import HorizontalScroller from "./HorizontalScroller";

interface Props {
  artist: ArtistI;
}
const Artist: Component<Props> = (props) => {
  const [works] = createResource(props.artist.id, commandSelectWorkByArtist, {
    initialValue: [],
  });

  const debugWorks = () => {
    const res = [];
    for (let i = 0; i < 10; i++) {
      res.push(...works());
    }
    return res;
  };

  return (
    <div class="w-full flex flex-col gap-2">
      <div class="font-bold text-lg">{props.artist.name}</div>
      <div class="">
        <HorizontalScroller
          isGradientFader={true}
          scrollStep={300}
          iconSize="md"
        >
          <div class="flex gap-4 p-4">
            <For each={debugWorks()}>
              {(work, i) => (
                <div class="w-32 h-32 flex-shrink-0">
                  <ArtistWork work={work} />
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
