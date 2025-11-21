<script setup lang="ts">
  import Navbar from './NavBar.vue'

  const location = document.pathname;
  const is_home = !(location == "/");
  const is_login = location == "/login";
  const is_register = location == "/register";

  const goBack = () => {
    history.back();
  }
</script>

<template>
  <NavBar>
    <button v-if="is_home" @click="goBack" class="button is-white">Go Back</button>
    <a v-if="is_login" class="button is-white" href="/register">Register</a>
    <a v-if="is_register" class="button is-white" href="/login">Login</a>
  </NavBar>
  <h1>You did it!</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>
</template>

<style scoped></style>
