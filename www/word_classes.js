"use strict";

const app = new Vue({
	el: "#app",
	data: {
		langs: [],
		word_classes: []
	}
});
const class_input = document.getElementById("class-input");
const invalid_class = document.getElementById("invalid-class");
const add_class_button = document.getElementById("add-class");

const update_classes = () => {
	fetch("/classes/English").then(response => {
		response.json().then(word_classes => {
			app.word_classes = [];
			for (let idx in word_classes) {
				app.word_classes.push({ name: word_classes[idx] });
			}
		});
	});
}

const post_class = () => {
	const url = "/class/English/" + class_input.value;
	fetch(url, { method: "POST" }).then(response => {
		if (response.status == 200) {
			invalid_class.style.visibility = "hidden";
			class_input.value = "";
		} else {
			invalid_class.style.visibility = "visible";
		}
		update_classes();
		class_input.select();
	});
}

class_input.addEventListener("keyup", function (event) {
	if (event.key === "Enter") {
		event.preventDefault();
		add_class_button.click();
	}
});

fetch("/langs").then(response => {
	response.json().then(langs => {
		app.langs = [];
		for (let idx in langs) {
			app.langs.push({ name: langs[idx] });
		}
	});
	update_classes();
	class_input.focus();
});
