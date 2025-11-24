<script setup lang="ts">
  import {defineProps, reactive, onMounted, defineEmits} from 'vue'
  import {useRouter} from 'vue-router'

  const router = useRouter();

  const emit = defineEmits(['childReply', 'childRepost']);

  const bodyData = reactive({
    liked: false,
    post: {}
  });

  const props = defineProps({
    post : Object,
  });

  const route = `/api/json/like/${props.post.owner}/${props.post.id}`;

  onMounted( async () => { try {
    const response = await fetch(route, {
      method : 'GET',
      headers : {"Content-Type": "application/json"},
      credentials: 'include'
    });

    const data = await response.json();

    bodyData.liked = data.did_like;
    bodyData.post = data.post;

  } catch(error) {
    console.log(error)
  }
  });

  async function like() {
    const response = await fetch(route, {
      method : 'PUT',
      headers : {"Content-Type": "application/json"},
      credentials: 'include'
    });

    const data = await response.json();

    bodyData.liked = data.did_like;
    bodyData.post = data.post;
  }

  function reply() {
    emit('childReply', `/api/json/reply/${bodyData.post.owner}/${bodyData.post.id}`);
  }

  function repost() {
    emit('childRepost', `/api/json/repost/${bodyData.post.owner}/${bodyData.post.id}`);
  }

  function goPost() {
    router.push(`/${bodyData.post.owner}/${bodyData.post.id}`);
  }

  function goRepost() {
    if(bodyData.post.repost) {
      const repost = bodyData.post.repost.split("////:////");

      router.push(`/${repost[0]}/${repost[1]}`);
    }
  }

</script>
<template>
  <div class="columns is-centered">
	<div class="column is-6-tablet is-5-desktop is-4-widescreen">
    <div class="box post-box" @click.self="goPost">
			<div class="field">
				<RouterLink class="subtitle is-5 nopropagate" v-bind:to="`/${post.owner}`">{{post.owner}}</RouterLink>
			</div>

			<div class="field">
				<p class="text">{{ post.contents }}</p>
			</div>
      <template  v-if="post.repost != null">

      <div class="box post-box nopropagate" @click.self="goRepost">

        <div class="field">
          <RouterLink class="subtitle is-5 nopropagate" v-bind:to="`/${post.repost.owner}`">{{ post.repost.owner }} • <span>{{ post.repost.total_likes }}</span> Likes</RouterLink>
        </div>

        <div class="field">
          <p class="text nopropagate">{{ post.repost.contents }}</p>
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
</template>
