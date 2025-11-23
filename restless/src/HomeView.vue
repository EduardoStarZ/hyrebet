<script setup lang="ts">
  import {onMounted, reactive} from 'vue'
  import PostComponent from './PostComponent.vue'

  const bodyData = reactive({
    username : '',
    total_posts : '',
    posts : []
  });

  onMounted( async () => {
    const response = await fetch('/api/json/', {
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
  <div class="columns is-centered">
    <div class="column is-6-tablet is-5-desktop is-5-widescreen my-5">
      <div class="box">

        <h1 class="subtitle is-3">Welcome {{ bodyData.username }}</h1>
        <p class="subtitle is-5">Total Posts In Database: {{ bodyData.total_posts }}</p>
        <div class="field buttons">
          <button class="button is-success post" v-bind:value="`${bodyData.username}/0`">Post</button>
          <a class="button is-danger" href="/auth/logout">Log Out</a>
        </div>
      </div>
    </div>
  </div>

  <div>
    <PostComponent v-for="data in bodyData.posts" :post="data" :key="`${data.owner}/${data.id}`" />

  </div>
</template>
<style scoped></style>
