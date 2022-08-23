import { Component } from "solid-js";
import type { Artist as ArtistI } from "../../lib/types";

interface Props {
  artist: ArtistI;
}
const Artist: Component<Props> = (props) => {
  return (
    <div>
      <div>artist compontnt</div>
      <div>iroirosuru</div>
    </div>
  );
};

export default Artist;
