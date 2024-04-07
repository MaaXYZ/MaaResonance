import AppConfig from "./AppConfig";

export interface MaaConfig {
    appConfig: AppConfig;
    combat: {
        times: number;
    };
    driveCombat: {
        use_fuel: boolean;
    };
}
