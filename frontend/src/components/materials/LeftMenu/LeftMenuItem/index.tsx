import { useMatch, useNavigate } from "react-router-dom";
import classes from "./classes.module.scss";
import { useCallback } from "react";

type IProps = {
    title: string;
    link: string;
    icon: JSX.Element;
};

export default function LeftMenuItem({ title, link, icon }: IProps) {
    const linkMatch = useMatch(link);
    const navigate = useNavigate();

    const navigateToLink = useCallback(() => {
        navigate(link);
    }, [link, navigate]);

    return (
        <div
            className={classes["root"]}
            data-active={(linkMatch !== null).toString()}
            onClick={navigateToLink}
        >
            <div className={classes["icon"]}>{icon}</div>
            <div className={classes["link"]}>{title}</div>
        </div>
    );
}
