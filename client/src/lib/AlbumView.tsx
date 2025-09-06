import {Album, Song} from "./types";

type Props = { album: Album, handlePlay : (Song) => void}

export default function AlbumView( {album, handlePlay} : Props) {

    return (
        <div>
            <b>Album: {album.name}</b>
            {album.songs?.map((song: Song) => (<button onClick={() => handlePlay(song)}>{song.name}</button>))}
        </div>
    )

}