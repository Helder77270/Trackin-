import { useEffect } from "react";
import classes from "./classes.module.scss";
import NavMenu from "../../materials/NavMenu";

type IProps = {
  children?: React.ReactNode;
  tabTitle?: string;
};

// npm i react-router-dom
export default function PageTemplate(props: IProps) {
  useEffect(() => {
    document.title = props.tabTitle || "React App";
  }, [props.tabTitle]);

  return (
    <div className={classes["root"]}>
      <header className={"menu"}>
        <NavMenu />
      </header>
      <main className={classes["body"]}>
        <div className={classes["content"]}>{props.children}</div>
      </main>
    </div>
  );
}
