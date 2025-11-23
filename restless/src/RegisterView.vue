<script setup lang="ts">
  import NavbarComponent from './NavbarComponent.vue'

  const handleSubmit = async () => {
    try {
        const response = await fetch('/json/eduardo', {
        method: 'GET',
        headers : {
          'Content-Type' : 'application/json'
        },
        });

        const value = await response.text();

        console.log(value);

      } catch (error) {
        console.error('Error submitting form:', error);
      }
  }

</script>

<template>
  <NavbarComponent>
    <RouterLink class="button is-white" to="/login">Login</RouterLink>
  </NavbarComponent>
  <h1>This is register</h1>
  <button @click="handleSubmit">This is some text</button>
</template>
