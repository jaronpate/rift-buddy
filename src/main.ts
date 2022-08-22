import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import Modal from './components/Modal.vue'


const Vue = createApp(App);
Vue.component('Modal', Modal)
Vue.mount('#app');
