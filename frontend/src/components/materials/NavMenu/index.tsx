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
        </div>
    );
}
