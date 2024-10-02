import { Link } from "react-router-dom";
import ModuleConfig from "../../../config/ModuleConfig";
import classes from "./classes.module.scss";
const pages = ModuleConfig.getInstance().getConfig().modules.pages;

export default function NavMenu() {
    return (
        <div className={classes["root"]}>
            <div className={classes["item"]}>
                <Link to={pages.Home.props.path}>Home</Link>
            </div>
            <div className={classes["item"]}>
                <Link to={pages.Test.props.path.replace(":seconds", "50")}>
                    Test
                </Link>
            </div>
            <div className={classes["item"]}>
                <Link to={pages.Users.props.path}>Users</Link>
            </div>
        </div>
    );
}
