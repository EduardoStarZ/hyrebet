let hash = new Hashes.SHA256;

const form = document.getElementById("form");

form.addEventListener("submit", function(event) {
	event.preventDefault();

	let password = document.getElementById('password');	

	value = password.value;

	password.value = hash.b64(value);
	
	form.submit();
})
