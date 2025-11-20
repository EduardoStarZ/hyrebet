function doubleEventListener(type, elements, func) {
	const attach = (elements, type, func) => { 
		elements.forEach(function(element) {
			element.addEventListener(type, func)
		})
	};
	
	document.addEventListener("htmx:afterSwap", function() {
		attach(elements, type, func);
	});
	
	attach(elements, type, func);
}

function closeModal() {
	let dialog = document.getElementById("postModal");

	dialog.close();
}

document.getElementById("postForm").addEventListener("submit", function() {
	setInterval(function() {
	let contents = document.getElementById("contents");
	contents.value = "";

	}, 3000);

	closeModal();
});

document.getElementById("cancel").addEventListener("click", closeModal);

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

let boxes = document.querySelectorAll(".post-box");

boxes.forEach(function(box) {
	box.addEventListener("click", function() {
		location.href = box.getAttribute("whereto");
	})	
});


function no_propagate() {
	let no_propagation_elts = document.querySelectorAll(".nopropagate");

	no_propagation_elts.forEach(function(elt) {
		elt.addEventListener("click", function(event) {
			event.stopPropagation();
		})
	})
}
