import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import mitt from "mitt";

const vue = createApp(App);

vue.config.globalProperties.$bus = mitt();

vue.mount("#app");
