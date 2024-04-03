import { createI18n } from "vue-i18n";
import enUS from "./localizations/en.json";

export const i18n = createI18n({
    locale: "en",
    messages: {
        "en": enUS,
    },
});
