import { Component, createResource, For } from "solid-js";
import { commandSelectWorkByArtist } from "../../lib/commands";
import type { Artist as ArtistI } from "../../lib/types";
import ArtistScroller from "./ArtistScroller";
import ArtistWork from "./ArtistWork";

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
        <ArtistScroller>
          <For each={debugWorks()}>
            {(work, i) => (
              <div class="w-32 h-32 flex-shrink-0">
                <ArtistWork work={work} />
              </div>
            )}
          </For>
        </ArtistScroller>
      </div>
    </div>
  );
};

export default Artist;
