import SongBox, { ISong } from "../SongBox";
import classes from "./classes.module.scss";

type IProps = {
    songs: ISong[];
    title?: string;
};

export default function SongsList({ songs, title }: IProps) {
    return (
        <div className={classes["root"]}>
            {title && <div className={classes["title"]}>{title}</div>}
            <div className={classes["songs-container"]}>
                {songs.map((song, index) => (
                    <SongBox key={index} song={song} />
                ))}
            </div>
        </div>
    );
}
