import {Album, Artist} from "./types";
import AlbumView from "./AlbumView";

type Props = { artist: Artist, handlePlay : (Song) => void }

export default function ArtistView({artist, handlePlay} : Props) {
    return (
        <div>
            <b>Name: {artist.name}</b>
            {artist.albums.map((album: Album) => (<AlbumView album={album} handlePlay={handlePlay}/>))}
        </div>
    )
}