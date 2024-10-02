import developmentConfig from "./development.json";
import productionConfig from "./production.json";

import stagingConfig from "./staging.json";

export default class ModuleConfig {
  private static instance: ModuleConfig;

  public static getInstance(): ModuleConfig {
    if (!ModuleConfig.instance) {
      ModuleConfig.instance = new ModuleConfig();
    }

    return ModuleConfig.instance;
  }

  private config: typeof developmentConfig = developmentConfig;

  private constructor() {
    this.setConfig();
  }

  public getConfig() {
    return this.config;
  }

  private setConfig() {
    switch (process.env["REACT_APP_ENV"]) {
      case "production":
        this.config = productionConfig;
        break;
      case "staging":
        this.config = stagingConfig;
        break;
    }
  }
}
