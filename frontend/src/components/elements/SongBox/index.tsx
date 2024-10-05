import classes from "./classes.module.scss";

export type ISong = {
    artist: string;
    title: string;
    image: string;
};

type IProps = {
    song: ISong;
};
export default function SongBox({ song }: IProps) {
    return (
        <div className={classes["root"]}>
            <div className={classes["image-container"]}>
                <img src={song.image} alt="song" />
            </div>
            <div className={classes["info"]}>
                <div className={classes["artist"]}>{song.artist}</div>
                <div className={classes["title"]}>{song.title}</div>
            </div>
        </div>
    );
}
