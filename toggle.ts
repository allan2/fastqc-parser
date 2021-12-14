let showFailOnly = false;
let totalCount = document.querySelectorAll('figure').length;
let passCount = document.querySelectorAll('.pass').length;
let numResults = totalCount;
let resultsCountEl = document.getElementById("results-count");

const showingMsg = (count: number): string => {
	let s = `Showing ${count} result`;
	if (count != 1) {
		s += "s";
	}
	return s
}

if (resultsCountEl !== null) {
	resultsCountEl.innerHTML = showingMsg(numResults);
}

const toggleShowFailOnly = () => {
	showFailOnly = !showFailOnly;
	let x = document.getElementsByClassName("pass") as HTMLCollectionOf<HTMLElement>;
	for (let i = 0; i < x.length; i++) {
		if (x[i].style.display == "none") {
			x[i].style.display = "block";
		} else {
			x[i].style.display = "none";
		}
	}

	if (resultsCountEl !== null) {
		let numResults: number;
		if (!showFailOnly) {
			numResults = totalCount;
		} else {
			numResults = passCount;
		}
		resultsCountEl.innerHTML = showingMsg(numResults);
	}
}
