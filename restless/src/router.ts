// router.js
import { createRouter, createMemoryHistory } from 'vue-router';
import { inject } from 'vue'
import HomeView from './HomeView.vue'
import LoginView from './LoginView.vue'
import RegisterView from './RegisterView.vue'
import type {VueCookies} from 'vue-cookies'

const $cookies = inject("$");

const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: '/',
      component: HomeView,
      meta: { requiresLogin: true },

    },
    {
      path: '/login',
      component: LoginView,
    },
    {
      path: '/register',
      component: RegisterView,
    },
  ],
});

router.beforeEach((to, from, next) => {
  const cookie = $cookies.get("Auth");

  if (to.meta.requiresLogin && cookie) {
    const token = cookie ?? "None";
    const isAuthenticated = token !== "None" ? true : false;
    if (isAuthenticated) {
      next(); // User is authenticated, proceed
    } else {
      next('/login'); // User is not authenticated, redirect to login
    }
  } else if (to.meta.requiresLogin){
    next('/login'); // Route does not require login, proceed
  }
});

export default router;
