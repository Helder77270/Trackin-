import { Link } from "react-router-dom";
import classes from "./classes.module.scss";
import Logo from "./logo.png";
import { WalletIcon } from "@heroicons/react/24/outline";
import User from "./user.png";
import ModuleConfig from "../../../config/ModuleConfig";
const pages = ModuleConfig.getInstance().getConfig().modules.pages;
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
            <div className={classes["right-container"]}>
                <Link to="/" className={classes["link"]}>
                    EXPLORE
                </Link>
                <WalletIcon className={classes["wallet-icon"]} />
                <Link to={pages.Profile.props.path}>
                    <img className={classes["user"]} src={User} alt="user" />
                </Link>
            </div>
        </div>
    );
}
