import { createApp } from 'vue'
import { createMemoryHistory, createRouter } from 'vue-router'
import App from './App.vue'
import LoginView from './LoginView.vue'
import RegisterView from './RegisterView.vue'
import axios from 'axios'

axios.defaults.headers.common = {
  "Access-Control-Allow-Origin" : "http://127.0.0.1:3000",
  "Access-Control-Allow-Methods" : "GET, PUT, POST",
  "Access-Control-Allow-Headers": "Origin, X-Requested-With, Content-Type, Accept"
}

axios.defaults.withCredentials = true;

const routes = [
  { path: '/', component: RegisterView },
  { path: '/login', component: LoginView},
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

const app = createApp(App).use(router);

app.mount('#app');
