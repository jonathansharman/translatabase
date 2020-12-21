"use strict";

const app = new Vue({
	el: "#app",
	data: {
		langs: []
	}
});
const lang_input = document.getElementById("lang-input");
const invalid_lang = document.getElementById("invalid-lang");
const add_lang_button = document.getElementById("add-lang");

const update_langs = () => {
	fetch("/langs").then(response => {
		response.json().then(langs => {
			app.langs = [];
			for (let idx in langs) {
				app.langs.push({ name: langs[idx] });
			}
		});
	});
}

const post_lang = () => {
	const url = "/lang/" + lang_input.value;
	fetch(url, { method: "POST" }).then(response => {
		if (response.status == 200) {
			invalid_lang.style.visibility = "hidden";
			lang_input.value = "";
		} else {
			invalid_lang.style.visibility = "visible";
		}
		update_langs();
		lang_input.select();
	});
}

lang_input.addEventListener("keyup", function (event) {
	if (event.key === "Enter") {
		event.preventDefault();
		add_lang_button.click();
	}
});

update_langs();
lang_input.focus();
