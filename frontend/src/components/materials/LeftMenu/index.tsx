import ModuleConfig from "../../../config/ModuleConfig";
import classes from "./classes.module.scss";
import LeftMenuItem from "./LeftMenuItem";
import {
    HomeIcon,
    DocumentPlusIcon,
    RectangleStackIcon,
} from "@heroicons/react/24/outline";

const pages = ModuleConfig.getInstance().getConfig().modules.pages;

export default function LeftMenu() {
    return (
        <div className={classes["root"]}>
            <LeftMenuItem
                icon={<HomeIcon />}
                link={pages.Home.props.path}
                title="Home"
            />
            <LeftMenuItem
                icon={<RectangleStackIcon />}
                link={pages.Created.props.path}
                title="Created"
            />
            <LeftMenuItem
                icon={<DocumentPlusIcon />}
                link={pages.Add.props.path}
                title="Add new"
            />
        </div>
    );
}
