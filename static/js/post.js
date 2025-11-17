function closeModal() {
	let dialog = document.getElementById("postModal");

	dialog.close();
}

document.getElementById("postForm").addEventListener("submit", function() {
	let contents = document.getElementById("contents");
	contents.value = "";

	closeModal();
});

document.getElementById("cancel").addEventListener("click", closeModal)

let buttons = document.querySelectorAll(".reply, .repost");

console.log(buttons);

buttons.forEach(function(button) {
	let is_repost = button.classList.contains("repost");
	let is_reply = button.classList.contains("reply");
		
	button.addEventListener("click", function() {
		let dialog = document.getElementById("postModal");

		let buttonValue = button.value;

		dialog.showModal();

		let form = document.getElementById("postForm");

		if (is_repost) {
			form.action = `http://127.0.0.1:4000/repost/${buttonValue}`;	
		} else if(is_reply) {
			form.action = `http://127.0.0.1:4000/reply/${buttonValue}`;
		} else {
			form.action = `http://127.0.0.1:4000/post/${buttonValue}`;
		}
	})
});
