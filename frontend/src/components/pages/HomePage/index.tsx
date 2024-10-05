import PageTemplate from "../PageTemplate";
import classes from "./classes.module.scss";
import userProfile from "../../materials/NavMenu/user.png";

export const user = {
    pseudo: "Dessert",
    firstName: "Cyril",
    lastName: "Rolland-Monnet",
};
export default function HomePage() {
    return (
        <PageTemplate tabTitle="Home">
            <div className={classes["root"]}>
                <div className={classes["welcome-container"]}>
                    <div className={classes["title"]}>
                        Welcome to Trackin, {user.pseudo}
                    </div>
                    <div className={classes["user-container"]}>
                        <div className={classes["left"]}>
                            <img src={userProfile} alt="user profile" />
                        </div>
                        <div className={classes["right"]}>
                            <div className={classes["pseudo"]}>
                                {user.pseudo},
                            </div>
                            <div className={classes["name"]}>
                                {user.firstName}&nbsp;
                                <span className={classes["last-name"]}>
                                    {user.lastName}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </PageTemplate>
    );
}
