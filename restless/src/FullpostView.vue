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
    post : {},
    replies : []
  });

  onMounted( async () => {
    const response = await fetch(`/api/json/${route.params.user}/${route.params.id}`, {
    method: 'GET',
    headers : {"Content-Type": "application/json"},
    credentials: 'include'
  });

    const data = await response.json();

    console.log(data);

    bodyData.post = data.post;
    bodyData.replies = data.replies;
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

  function reply() {
    openDialog(`/api/json/reply/${bodyData.post.owner}/${bodyData.post.id}`);
  }

  function repost() {
    openDialog(`/api/json/repost/${bodyData.post.owner}/${bodyData.post.id}`);
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
  <div class="column is-6-tablet is-5-desktop is-5-widescreen">
    <div class="box post-box" whereto="/{post.owner}/{post.id}?fullview=true">
      <div class="field">
        <p class="subtitle is-5 nopropagate">{{bodyData.post.owner}}</p>
      </div>

      <div class="field">
        <p class="text">{{ bodyData.post.contents }}</p>
      </div>
      <template  v-if="bodyData.post.repost != null">

        <div class="box post-box nopropagate">
          <div class="field">
            <RouterLink class="subtitle is-5 nopropagate" v-bind:to="`/${bodyData.post.repost.owner}`">{{ bodyData.post.repost.owner }} • <span>{{bodyData.post.repost.total_likes }}</span> Likes</RouterLink>
          </div>

          <div class="field">
            <p class="text nopropagate">{{ bodyData.post.repost.contents }}</p>
          </div>
        </div>

      </template>

      <div class="field buttons">
        <button class="button is-success reply nopropagate" @click="reply">Reply</button>
        <button class="button is-success repost nopropagate" @click="repost">Repost</button>

        <div class="control has-icons-left">
          <button class="button is-danger like-button" v-if="bodyData.liked" @click="like" >{{ bodyData.post.total_likes }} Likes</button>
          <button class="button is-danger is-outlined like-button" v-else @click="like">{{ bodyData.post.total_likes }} Likes</button>
        </div>
      </div>
    </div>
  </div>
  </div>

  <div>
    <PostComponent v-for="data in bodyData.replies" :post="data" :key="`${data.owner}/${data.id}`"
    @child-reply="openDialog" @child-repost="openDialog"/>
  </div>

  	<dialog ref="dialog">
			<form @submit.prevent="submit">
				<div class="field buttons">
					<input type="submit" class="button is-success" value="Post" />
					<input type="button" class="button is-danger" @click="closeDialog" value="Cancel" />
				</div>

				<div class="control">
					<textarea class="textarea is-normal has-fixed-size" v-model="formData.contents" @keyup.ctrl.enter="submit" rows="5" cols="80" maxlenght="240" required autofocus></textarea>
				</div>
			</form>
		</dialog>
</template>
<style scoped></style>
