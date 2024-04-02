import { createApp } from "vue";
import MiniWindow from "./MiniWindow.vue";
import { createPinia } from "pinia";
import "../styles/styles.css"
import "../styles/theme.css"
import 'mdui/mdui.css'
import { setupListener } from "@/CallbackListner";

const app = createApp(MiniWindow);

const pinia = createPinia();
app.use(pinia);

setupListener();

app.mount("#miniwindow");
