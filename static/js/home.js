let goback = document.getElementById("back");

setInterval(function() {
	if (location.pathname == "/" || location.pathname == "") {
		goback.classList.add("nodisplay");
	} else {
		goback.classList.remove("nodisplay");
	}
}, 0);

goback.addEventListener("click", function() {
	history.back();
	window.location.reload();
})
