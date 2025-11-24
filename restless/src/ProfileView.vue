<script setup lang="ts">
  import {onMounted, reactive, ref} from 'vue'
  import PostComponent from './PostComponent.vue'
  import NavbarComponent from './NavbarComponent.vue'
  import { useRoute, useRouter } from 'vue-router'

  const route = useRoute();

  const router = useRouter();

  const formData = ref({
    contents: ''
  });

  const bodyData = reactive({
    username : '',
    total_posts : '',
    posts : []
  });

  onMounted( async () => {
    const response = await fetch(`/api/json/${route.params.user}`, {
    method: 'GET',
    headers : {"Content-Type": "application/json"},
    credentials: 'include'
  });

    const data = await response.json();

    console.log(data);

    console.log(data.username);
    console.log(data.total_posts);

    bodyData.username = data.username;
    bodyData.total_posts = data.total_posts;
    bodyData.posts = data.posts;
  });

  const location = ref("");

  const dialog = ref(null);

  const openDialog = (data) => {
    location.value = data;

    console.log(location.value);

    dialog.value.showModal() ;
  }

  const closeDialog = () => {
    dialog.value.close();
  }

  const submit = async () => {
   const response = await fetch(location.value, {
      method: 'POST',
      headers : {"Content-Type": "application/json"},
      credentials: 'include',
      body: JSON.stringify(formData.value)
    });

    console.log(response);

    closeDialog();

    router.go(0);

  }
</script>
<template>
  <NavbarComponent>
    <button class="button is-white" @click="$router.back()">Go Back</button>
  </NavbarComponent>
  <div class="columns is-centered">
    <div class="column is-6-tablet is-5-desktop is-5-widescreen my-5">
      <div class="box">
        <h1 class="subtitle is-3">{{ bodyData.username }}</h1>
        <p class="subtitle is-5">Posts: {{ bodyData.total_posts }}</p>
      </div>
    </div>
  </div>

  <div>
    <PostComponent v-for="data in bodyData.posts" :post="data" :key="`${data.owner}/${data.id}`"
    @child-reply="openDialog" @child-repost="openDialog"/>
  </div>

  	<dialog ref="dialog">
			<form @submit.prevent="submit">
				<div class="field buttons">
					<input type="submit" class="button is-success" value="Post" />
					<input type="button" class="button is-danger" @click="closeDialog" value="Cancel" />
				</div>

				<div class="control">
					<textarea class="textarea is-normal has-fixed-size" @keyup.ctrl.enter="submit" v-model="formData.contents" rows="5" cols="80" maxlenght="240" required autofocus></textarea>
				</div>
			</form>
		</dialog>
</template>
<style scoped></style>
