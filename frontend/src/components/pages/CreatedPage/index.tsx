import SongsList from "../../elements/SongsList";
import PageTemplate from "../PageTemplate";
import classes from "./classes.module.scss";

export default function CreatedPage() {
    return (
        <PageTemplate tabTitle="Created">
            <div className={classes["root"]}>
                <div className={classes["title"]}>Your library</div>
                <div className={classes["songs-container"]}>
                    <SongsList
                        songs={[
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights feat ifjfjepfiodejzfiopjk foejzfipezjfi jfiopejzpf",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                            {
                                artist: "The Weeknd",
                                title: "Blinding Lights",
                                image: "https://upload.wikimedia.org/wikipedia/en/e/e6/The_Weeknd_-_Blinding_Lights.png",
                            },
                        ]}
                    />
                </div>
            </div>
        </PageTemplate>
    );
}
