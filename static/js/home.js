setInterval(function() {
	let goback = document.getElementById("navbar");

	if (location.pathname == "/" || location.pathname == "") {
		goback.classList.add("nodisplay");
	} else {
		goback.classList.remove("nodisplay");
	}
}, 200);

let goback_button = document.getElementById("back");

goback_button.addEventListener("click", function() {
	history.back();
	window.location.reload();
})
