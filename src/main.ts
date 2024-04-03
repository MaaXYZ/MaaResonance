import { createApp } from "vue";
import "./styles/styles.css";
import "./styles/theme.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import { setupListener } from "./CallbackListner";
import ToastPlugin from 'vue-toast-notification';

import 'mdui/mdui.css';

import 'mdui/components/button'
import 'mdui/components/circular-progress'
import 'mdui/components/button-icon'
import "mdui/components/card";
import 'mdui/components/linear-progress'
import 'mdui/components/tabs'
import 'mdui/components/tab'
import 'mdui/components/tab-panel'
import 'mdui/components/dropdown'
import 'mdui/components/menu'
import 'mdui/components/menu-item'

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

app.use(ToastPlugin)

setupListener();

app.mount("#app");
