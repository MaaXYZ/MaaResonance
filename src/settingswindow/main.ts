import { createApp } from "vue";
import Settings from "./Settings.vue";
import "mdui/mdui.css";
import "../styles/styles.css";
import "../styles/theme.css";

import "mdui/components/list";
import "mdui/components/list-item";
import "mdui/components/select";
import "mdui/components/menu-item";
import { createPinia } from "pinia";
import { setupListener } from "@/CallbackListner";
import { i18n } from "@/I18n";

const app = createApp(Settings);
const pinia = createPinia();
app.use(pinia);

app.use(await i18n());

setupListener();

app.mount("#settingswindow");
