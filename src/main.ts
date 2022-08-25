import { createApp } from 'vue'
import './style.css'
import './assets/css/main.less'
import App from './App.vue'
import Modal from './components/Modal.vue'


const Vue = createApp(App);
Vue.component('Modal', Modal)
Vue.mount('#app');
