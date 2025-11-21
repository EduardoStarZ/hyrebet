<script setup lang="ts">
import { ref } from 'vue';

const home = ref(location.pathname);

setInterval(function() {
  home.value = location.pathname;
}, 100);

const goBack = () => {
  history.back();
}
</script>

<template>

  <nav id="navbar" class="navbar" role="navigation" aria-label="main navigation">
    <h1 class="title is-3 mx-4 my-2 logo">Hyrebet</h1>

    <div class="navbar-menu">
      <div class="navbar-start">
        <div class="navbar-item">
          <div id="left-buttons" class="buttons">
            <button v-if="home == true" id="back" class="nodisplay"></button>
            <button v-else @click="goBack" id="back" class="button is-white">Go back</button>
          </div>
        </div>
      </div>
    </div>
  </nav>

</template>

<style scoped>
.navbar {
		display: flex !important;
		background-color: #48C78E !important;
}

.navbar-menu {
		flex-grow: initial !important;
		flex-shrink: initial !important;
}

.navbar, .navbar-brand {
    height: max-content !important;
}

.navbar-center {
	display: flex !important;
    align-self: center;
    z-index: 200;
    margin: 0 auto;
}

.navbar {
    width: 100% !important;
    align-content: center !important;
}

.logo {
	font-family: cookie;
	pointer-events: none !important;
	-webkit-user-select: none;
	-ms-user-select: none;
	user-select: none;
}

.nodisplay {
	display : none !important;
}
</style>
