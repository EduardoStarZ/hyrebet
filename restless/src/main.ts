import { createApp } from 'vue'
import App from './App.vue'
import NavbarComp from './NavbarComp.vue'


const index = createApp(App);

const navbar = createApp(NavbarComp);

index.mount('#app');
navbar.mount('#navbar-container');
