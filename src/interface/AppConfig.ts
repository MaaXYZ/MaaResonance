export const maaAdbControllerTouchTypes = [
    "MaaTouch",
    "MiniTouch",
    "Adb",
    "AutoDetect",
] as const;

export type MaaAdbControllerTouchType = typeof maaAdbControllerTouchTypes[number];

export const maaAdbControllerKeyTypes = ["MaaTouch", "Adb", "AutoDetect"] as const;

export type MaaAdbControllerKeyType = typeof maaAdbControllerKeyTypes[number];

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

export type MaaAdbControllerScreencapType = typeof maaAdbControllerScreencapTypes[number];

export interface MaaAdbControllerType {
    touch_type: MaaAdbControllerTouchType;
    key_type: MaaAdbControllerKeyType;
    screencap_type: MaaAdbControllerScreencapType;
}

export default interface AppConfig {
    adb_controller_type: MaaAdbControllerType;
}
