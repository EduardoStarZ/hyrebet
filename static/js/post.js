function invokeAfterSwap(func) {
	document.body.addEventListener("htmx:afterSwap", func);

	func();
}

function closeModal() {
	let dialog = document.getElementById("postModal");

	dialog.close();
}

document.getElementById("postForm").addEventListener("submit", function() {
	closeModal();
	setTimeout(function() {
	let contents = document.getElementById("contents");
	contents.value = "";

	}, 300);
});

document.body.addEventListener('keydown', (event) => {
	let dialog = document.getElementById("postModal");

	let textarea = document.getElementById("contents");

	let form = document.getElementById("postForm");

	if (dialog.open && textarea.value.trim() != '') {
		if(event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
			event.preventDefault();
			form.requestSubmit();
		}
	}

});

document.getElementById("cancel").addEventListener("click", closeModal);


function setPost() {
	let buttons = document.querySelectorAll(".reply, .repost, .post");

	buttons.forEach(function(button) {
		let is_repost = button.classList.contains("repost");
		let is_reply = button.classList.contains("reply");

		button.addEventListener("click", function() {
			let dialog = document.getElementById("postModal");

			let buttonValue = button.value;

			dialog.showModal();

			let form = document.getElementById("postForm");

			if (is_repost) {
				form.setAttribute("hx-post", `http://127.0.0.1:4000/repost/${buttonValue}`);	
			} else if(is_reply) {
				form.setAttribute("hx-post", `http://127.0.0.1:4000/reply/${buttonValue}`);
			} else {
				form.setAttribute("hx-post", `http://127.0.0.1:4000/post/${buttonValue}`);
			}

			htmx.process(form);
		})
	});
}

function redo_boxes() {
	let boxes = document.querySelectorAll(".post-box");
	boxes.forEach(function(box) {
		box.addEventListener("click", function() {
			location.href = box.getAttribute("whereto");
		})	
	});
}

function cancel_propagation() {
	let no_propagation_elts = document.querySelectorAll(".nopropagate, .like-button");
	no_propagation_elts.forEach(function(elt) {
		elt.addEventListener("click", function(event) {
			event.stopPropagation();
		})
	})
}

invokeAfterSwap(setPost);
invokeAfterSwap(redo_boxes);
invokeAfterSwap(cancel_propagation);
