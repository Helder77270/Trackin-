import classes from "./classes.module.scss";
import Logo from "./logo.png";

export default function NavMenu() {
    return (
        <div className={classes["root"]}>
            <div className={classes["logo-container"]}>
                <img className={classes["logo"]} src={Logo} alt="logo" />
            </div>
            <div className={classes["input-container"]}>
                <input
                    type="text"
                    placeholder="Search for tokens, collections and users"
                    className={classes["input"]}
                />
            </div>
            <div className={classes["right-container"]}></div>
        </div>
    );
}
