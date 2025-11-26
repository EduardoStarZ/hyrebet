let hash = new Hashes.SHA256;

const form = document.getElementById("form");

form.addEventListener("submit", function(event) {
	event.preventDefault();

	let password1 = document.getElementById('password1');	

	value = password1.value;

	password1.value = hash.b64(value);

	let password2 = document.getElementById('password2');	

	value = password2.value;

	password2.value = hash.b64(value);

	
	form.submit();
})
