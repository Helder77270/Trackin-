// npm i sass

import classes from "./classes.module.scss";
export enum EColor {
  RED = "red",
  GREEN = "green",
  BLUE = "blue",
  GRAY = "gray",
}

type IProps = {
  title: string;
  color?: EColor;
};

export default function Title(props: IProps) {
  const { color = EColor.BLUE, title } = props;
  return (
    <h1 className={classes.root} style={{ color: color }}>
      {title}
    </h1>
  );
}
