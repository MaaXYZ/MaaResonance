export const maaAdbControllerTouchTypes = [
    "MaaTouch",
    "MiniTouch",
    "Adb",
    "AutoDetect",
] as const;

export type MaaAdbControllerTouchType =
    (typeof maaAdbControllerTouchTypes)[number];

export const maaAdbControllerKeyTypes = [
    "MaaTouch",
    "Adb",
    "AutoDetect",
] as const;

export type MaaAdbControllerKeyType = (typeof maaAdbControllerKeyTypes)[number];

export const maaAdbControllerScreencapTypes = [
    "FastestWayCompatible",
    "RawByNetcat",
    "RawWithGzip",
    "Encode",
    "EncodeToFile",
    "MinicapDirect",
    "MinicapStream",
    "FastestLosslessWay",
    "FastestWay",
] as const;

export type MaaAdbControllerScreencapType =
    (typeof maaAdbControllerScreencapTypes)[number];

export interface MaaAdbControllerType {
    touch_type: MaaAdbControllerTouchType;
    key_type: MaaAdbControllerKeyType;
    screencap_type: MaaAdbControllerScreencapType;
}

export const allDestinations = [
    "7号自由港",
    "曼德矿场",
    "修格里城",
    "阿妮塔能源研究所",
    "荒原站",
] as const;

export type Destination = (typeof allDestinations)[number];

export interface MaaConfig {
    appConfig: {
        adb_controller_type: MaaAdbControllerType;
    };
    combat: {
        times: number;
    };
    travel: {
        destination: Destination;
    };
}
