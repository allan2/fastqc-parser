const toggleShowFailOnly = () => {
	let x = document.getElementsByClassName("pass") as HTMLCollectionOf<HTMLElement>;
	for (let i = 0; i < x.length; i++) {
		if (x[i].style.display == "none") {
			x[i].style.display = "block";
		} else {
			x[i].style.display = "none";
		}
	}
}
