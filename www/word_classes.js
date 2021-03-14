"use strict";

const app = {
	data() {
		return {
			langs: [],
			word_classes: []
		}
	},
	mounted() {
		const class_input = document.getElementById("class-input");
		const invalid_class = document.getElementById("invalid-class");
		const add_class_button = document.getElementById("add-class");

		const update_classes = () => {
			fetch("/classes/English").then(response => {
				response.json().then(word_classes => {
					this.word_classes = word_classes;
				});
			});
		}

		const post_class = () => {
			const options = {
				method: "POST",
				headers: { 'Content-Type': 'application/text' },
				body: class_input.value
			}
			fetch("/classes/English", options).then(response => {
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
				this.langs = langs;
			});
			update_classes();
			class_input.focus();
		});
	}
};

Vue.createApp(app).mount("#app");
