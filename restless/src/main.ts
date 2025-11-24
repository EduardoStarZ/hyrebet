import { createApp } from 'vue'
import { createWebHistory, createRouter } from 'vue-router'
import App from './App.vue'
import LoginView from './LoginView.vue'
import RegisterView from './RegisterView.vue'
import VueCookies from 'vue-cookies'
import HomeView from './HomeView.vue'
import ProfileView from './ProfileView.vue'
import FullpostView from './FullpostView.vue'

const routes = [
  { path: '/', component: RegisterView },
  { path: '/login', component: LoginView},
  { path: '/home', component: HomeView},
  { path: '/:user', component: ProfileView},
  { path: '/:user/:id', component: FullpostView}
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

const app = createApp(App).use(router).use(VueCookies);

app.mount('#app');
