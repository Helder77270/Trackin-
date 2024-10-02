import React, { useRef, useState } from "react";

type IProps = {
  seconds: number;
};

export default function Timer(props: IProps) {
  const [seconds, setSeconds] = useState(props.seconds);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const propsRef = useRef(props);

  if (propsRef.current.seconds !== props.seconds) {
    setSeconds(props.seconds);
    propsRef.current = props;
  }
  // Fonction pour démarrer le timer
  const startTimer = () => {
    if (intervalRef.current !== null) {
      clearInterval(intervalRef.current);
    }

    intervalRef.current = setInterval(() => {
      setSeconds((prevState) => {
        if (prevState === 0) {
          clearInterval(intervalRef.current!);
          return 0;
        } else {
          return prevState - 1;
        }
      });
    }, 1000);
  };

  // Démarrage du timer au premier rendu
  if (intervalRef.current === null) {
    startTimer();
  }

  return <div>{seconds} left</div>;
}
