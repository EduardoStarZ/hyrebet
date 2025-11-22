<script setup lang="ts">
  import { ref } from 'vue'
  import NavbarComponent from './NavbarComponent.vue'
  import axios from 'axios';

  const formData = ref({
    username: '',
    password: ''
  });

  const handleSubmit = async () => {
    try {
        const response = await axios.post('http://127.0.0.1:3000/login', formData.value);

        console.log('Form submitted successfully:', response.data);

        formData.value = { name: '', email: '' };
      } catch (error) {
        console.error('Error submitting form:', error);
      }
  }
</script>

<template>
  <NavbarComponent>
    <RouterLink class="button is-white" to="/">Register</RouterLink>
  </NavbarComponent>

<section class="hero is-fullbright">
			<div class="hero-body">
				<div class="container">
					<div class="columns is-centered">

						<div class="column is-6-tablet is-5-desktop is-4-widescreen">

							<form class="box my-6" @submit.prevent="handleSubmit">
								<h1 class="title">Please Login</h1>

								<div class="field">

									<label for="username" class="label mt-5 mx-auto">Username</label>

									<div class="control has-icons-left">
										<input type="text" class="input" v-model="formData.username" placeholder="Username" required/>
										<span class="icon is-small is-left">
											<i class="fa fa-user"></i>
										</span>
									</div>
								</div>

								<div class="field">

									<label for="password" class="label mx-auto">Password</label>

									<div class="control has-icons-left">
										<input type="password" class="input" v-model="formData.password" placeholder="***********" required/>
										<span class="icon is-small is-left">
											<i class="fa fa-lock"></i>
										</span>
									</div>
								</div>

								<p>Don't have an account? <RouterLink class="link" to="/register">Register</RouterLink></p>


								<div class="field buttons my-4">
									<input type="submit" class="button is-success is-fullwidth" id="confirm" value="Confirm" />
								</div>

							</form>
						</div>

					</div>

				</div>
			</div>
		</section>

</template>
