import { createApp } from "vue";
import MiniWindow from "./MiniWindow.vue";
import { createPinia } from "pinia";
import "../styles/styles.css"
import "../styles/theme.css"
import 'mdui/mdui.css'
import { setupListener } from "@/CallbackListner";
import { i18n } from "@/I18n";

const app = createApp(MiniWindow);

const pinia = createPinia();
app.use(pinia);

app.use(await i18n());

setupListener();

app.mount("#miniwindow");
