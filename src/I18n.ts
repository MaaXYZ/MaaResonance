import { createI18n } from "vue-i18n";
import { locale } from '@tauri-apps/plugin-os'
import en from "./localizations/en.json";
import zh from "./localizations/zh.json";

export const i18n =async () => {

    const lang = await locale();

    return createI18n({
        legacy: false,
        locale: lang || "en",
        fallbackLocale: "en",
        messages: {
            en: en,
            zh: zh,
        },
    });
}
