export const allClientTypes = ["Official", "Bilibili"] as const;

export type ClientType = (typeof allClientTypes)[number];

export interface StartUpConfig {
    clientType: ClientType;
}
