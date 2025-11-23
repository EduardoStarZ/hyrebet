<script setup lang="ts">
  import NavbarComponent from './NavbarComponent.vue'
  import {ref} from 'vue'
  import {useRouter} from 'vue-router'

  const formData = ref({
    username : '',
    password1 : '',
    password2 : ''
  })

  const router = useRouter()

  const handleSubmit = async () => {
    try {
        const response = await fetch('/auth/json/register', {
        method: 'POST',
        headers : {
          'Content-Type' : 'application/json'
        },
        body: JSON.stringify(formData.value)
        });

        if(response.ok) {
          router.push("/login")
        }

      } catch (error) {
        console.error('Error submitting form:', error);
      }
  }

</script>

<template>
  <NavbarComponent>
    <RouterLink class="button is-white" to="/login">Login</RouterLink>
  </NavbarComponent>
  <section class="hero is-fullbright">
			<div class="hero-body">
				<div class="container">
					<div class="columns is-centered">

						<div class="column is-6-tablet is-5-desktop is-4-widescreen">

							<form class="box my-6" @submit.prevent="handleSubmit">
								<h1 class="title">Create An Account</h1>

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

									<label for="password1" class="label mx-auto">Password</label>

									<div class="control has-icons-left">
										<input type="password" class="input" v-model="formData.password1" placeholder="***********" required/>
										<span class="icon is-small is-left">
											<i class="fa fa-lock"></i>
										</span>
									</div>
								</div>

								<div class="field">

									<label for="password2" class="label mx-auto">Confirm your password</label>

									<div class="control has-icons-left">
										<input type="password" class="input" v-model="formData.password2" placeholder="***********" required/>
										<span class="icon is-small is-left">
											<i class="fa fa-lock"></i>
										</span>
									</div>
								</div>

								<p>Already have an account? <RouterLink class="link" to="/login">Login</RouterLink></p>

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
