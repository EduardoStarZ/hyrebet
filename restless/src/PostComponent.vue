<script setup lang="ts"></script>
<template>
  <div class="columns is-centered">
	<div class="column is-6-tablet is-5-desktop is-4-widescreen">
		<div class="box post-box" whereto="/{post.owner}/{post.id}?fullview=true">
			<div class="field">
				<RouterLink class="subtitle is-5 nopropagate" v-bind:to="`/${post.owner}`">{{post.owner}}</RouterLink>
			</div>

			<div class="field">
				<p class="text">{{post.contents}}</p>
			</div>

			<div class="box post-box nopropagate" v-if="post.repost !== null" v-bind:whereto="`/${post.repost.owner}/${post.repost.id}?fullview=true`">

				<div class="field">
          <RouterLink class="subtitle is-5 nopropagate" v-bind:to="`/${post.repost.owner}`">{{ post.repost.owner }} • <span>{{ post.repost.total_likes }}</span>likes</RouterLink>
				</div>

				<div class="field">
					<p class="text nopropagate">{{ post.repost.contents }}</p>
				</div>
			</div>

			{% when None %}
			{% endmatch %}


			<div class="field buttons">
				<button class="button is-success reply nopropagate" v-bind:value="`${post.owner}/${post.id}`">Reply</button>
				<button class="button is-success repost nopropagate" v-bind:value="`${post.owner}/${post.id}`">Repost</button>

				<div class="control has-icons-left">
					<button class="button is-danger is-outlined like-button" v-if="didLike">Like</button>
				</div>
			</div>
		</div>
	</div>
</div>
</template>
