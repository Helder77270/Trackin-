import React from "react";

type IProps = {
  seconds: number;
};

type IState = {
  secondsLeft: number;
};

export default class TimerClass extends React.Component<IProps, IState> {
  private interval: NodeJS.Timeout | null = null;

  constructor(props: IProps) {
    super(props);
    this.state = {
      secondsLeft: props.seconds,
    };
  }

  public componentDidMount(): void {
    this.interval = setInterval(() => {
      if (this.state.secondsLeft === 0) {
        clearInterval(this.interval!);
        return;
      }
      this.setState((prevState) => ({
        secondsLeft: prevState.secondsLeft - 1,
      }));
    }, 1000);
  }

  public componentWillUnmount(): void {
    if (this.interval !== null) {
      clearInterval(this.interval);
    }
  }

  public render() {
    return <div>Temps restant : {this.state.secondsLeft} seconds </div>;
  }
}
