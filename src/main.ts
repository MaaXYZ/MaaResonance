import { createApp } from "vue";
import "./styles/styles.css";
import "./styles/theme.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import { setupListener } from "./CallbackListner";
import "./MduiImports";
import { i18n } from "./I18n";

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

app.use(await i18n());

setupListener();

app.mount("#app");
