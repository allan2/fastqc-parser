// mutable state
let showFailOnly = false;
const getCounts = (divId: string): [number, number] => {
	const d = document.getElementById(divId) as HTMLDivElement;
	console.log(divId);
	return [d.querySelectorAll('figure').length, d.querySelectorAll('.fail').length];
}
let [totalCount, failCount] = getCounts("before-trim-content");

let numResults = totalCount;

let resultsCountEl = document.getElementById("results-count") as HTMLSpanElement;

const showingMsg = (n: number): string => {
	let s = `Showing ${n} result`;
	if (n != 1) {
		s += "s";
	}
	return s
}

resultsCountEl.innerHTML = showingMsg(numResults);

const toggleShowFailOnly = () => {
	showFailOnly = !showFailOnly;
	const x = document.getElementsByClassName("pass") as HTMLCollectionOf<HTMLElement>;
	for (let i = 0; i < x.length; i++) {
		toggleVisible(x[i]);
	}

	if (!showFailOnly) {
		numResults = totalCount;
	} else {
		numResults = failCount;
	}
	console.log(numResults);
	resultsCountEl.innerHTML = showingMsg(numResults);
}

const toggleVisible = (el: HTMLElement) => (el.style.display == "none") ? el.style.display = "block" : el.style.display = "none";

let checkBox = document.getElementById("check") as HTMLInputElement;
checkBox.addEventListener("change", toggleShowFailOnly);

const openTab = (e: Event) => {
	// Reset show fail only.
	showFailOnly = false;
	checkBox.checked = false;

	// Hide all tab content elements.
	let tabContentEls = document.getElementsByClassName("tab-content") as HTMLCollectionOf<HTMLDivElement>;
	for (let i = 0; i < tabContentEls.length; i++) {
		tabContentEls[i].style.display = "none";
	}
	// Remove the active class from the tab buttons.
	let tabBtns = document.getElementsByClassName("tab-btn") as HTMLCollectionOf<HTMLButtonElement>;
	for (let i = 0; i < tabBtns.length; i++) {
		tabBtns[i].classList.remove("active");
		tabBtns[i].disabled = false;
	}

	let currEl = e.target as HTMLButtonElement;
	let contentId = "before-trim-content";
	if (currEl.id === "after-trim-btn") {
		contentId = "after-trim-content";
	}
	// Update the counts.
	[totalCount, failCount] = getCounts(contentId);
	// Show the tab's content.
	let content = document.getElementById(contentId) as HTMLDivElement;
	content.style.display = "block";
	// Make the tab button active.
	currEl.classList.add("active");
	currEl.disabled = true;
}

const tabBtns = document.querySelectorAll(".tab-btn");
for (let i = 0; i < tabBtns.length; i++) {
	tabBtns[i].addEventListener("click", openTab);
}


