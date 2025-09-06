import {Artist} from "./types";
import ArtistView from "./ArtistView";

type Props = { artists: Array<Artist>, handlePlay : (Song) => void  };

export default function Library( {artists, handlePlay} : Props) {

    return (
        <div>
            <h2>Artists</h2>
            { artists.map((artist: Artist) => (<ArtistView artist={artist} handlePlay={handlePlay}/>)) }
        </div>
    )
}
