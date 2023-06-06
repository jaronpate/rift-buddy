import { createApp } from "vue";
import "./style.css";
import "./assets/css/main.less";
import App from "./App.vue";
import Modal from "./components/Modal.vue";
import Prohibition from "./components/icons/Prohibition.vue";
import Download from "./components/icons/Download.vue";
import Upload from "./components/icons/Upload.vue";
import Trash from "./components/icons/Trash.vue";

const Vue = createApp(App);
Vue.component("Trash", Trash);
Vue.component("Upload", Upload);
Vue.component("Download", Download);
Vue.component("Prohibition", Prohibition);
Vue.component("Modal", Modal);
Vue.mount("#app");
