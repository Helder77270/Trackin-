import React, { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import ModuleConfig from "../../../config/ModuleConfig";

type IProps = {
  from: {
    enabled: boolean;
    props?: {
      [key: string]: unknown;
    };
  };
  children: React.ReactNode;
};

const pages = ModuleConfig.getInstance().getConfig().modules.pages;

export default function ModulePage(props: IProps) {
  const navigate = useNavigate();

  useEffect(() => {
    if (!props.from.enabled) {
      navigate(pages.Home.props.path);
    }
  }, [navigate, props.from.enabled]);
  if (!props.from.enabled) return null;
  return <React.Fragment>{props.children}</React.Fragment>;
}
