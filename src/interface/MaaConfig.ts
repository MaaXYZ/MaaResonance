import AppConfig from "./AppConfig";
import { StartUpConfig } from "./StartUpConfig";

export interface MaaConfig {
    startUp: StartUpConfig;
    appConfig: AppConfig;
}
