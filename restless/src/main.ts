import { createApp } from 'vue'
import { createMemoryHistory, createRouter } from 'vue-router'
import App from './App.vue'
import LoginView from './LoginView.vue'
import RegisterView from './RegisterView.vue'
import VueCookies from 'vue-cookies'


const routes = [
  { path: '/', component: RegisterView },
  { path: '/login', component: LoginView},
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

const app = createApp(App).use(router).use(VueCookies);

app.mount('#app');
