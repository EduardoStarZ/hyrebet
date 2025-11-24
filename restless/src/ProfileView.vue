<script setup lang="ts">
  import {onMounted, reactive} from 'vue'
  import PostComponent from './PostComponent.vue'
  import NavbarComponent from './NavbarComponent.vue'

  const bodyData = reactive({
    username : '',
    total_posts : '',
    posts : []
  });

  onMounted( async () => {
    const response = await fetch(`/api/json/${bodyData.username}`, {
    method: 'GET',
    headers : {"Content-Type": "application/json"},
    credentials: 'include'
  });

    const data = await response.json();

    console.log(data);

    bodyData.username = data.username;
    bodyData.total_posts = data.total_posts;
    bodyData.posts = data.posts;
  })
</script>
<template>
  <NavbarComponent />
  <div class="column is-6-tablet is-5-desktop is-5-widescreen my-5">
    <div class="box">
      <h1 class="subtitle is-3">{{ bodyData.username }}</h1>
      <p class="subtitle is-5">Posts: {{ bodyData.total_posts }}</p>
    </div>
  </div>

  <div>
    <PostComponent v-for="data in bodyData.posts" :post="data" :key="`${data.owner}/${data.id}`" />
  </div>
</template>
<style scoped></style>
