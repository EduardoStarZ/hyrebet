<script setup lang="ts">
  async function tryMe() {
  try {
  const response = await fetch('/api/json/', {
    method: 'GET',
    headers : {"Content-Type": "application/json"},
    credentials: 'include'
  });

  const data = await response.text();

  console.log(data);
  } catch(error) {
    console.log(error);
  }
  }
</script>
<template>
    <h1>Hold Up</h1>
  <div class="columns is-centered">
    <div class="column is-6-tablet is-5-desktop is-5-widescreen my-5">
      <div class="box">

      </div>
    </div>
  </div>

  <div>

  </div>
</template>
<style scoped></style>
