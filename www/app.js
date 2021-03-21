"use strict";

const app = {
	data() {
		return {
			panel: "langs",
			langs: [],
			renaming_lang: {},
			word_classes: [],
			error: null
		};
	},
	methods: {
		// Languages
		update_langs() {
			fetch("/langs").then(response => {
				response.json().then(langs => {
					this.langs = langs;
				});
			});
		},
		start_renaming(lang) {
			this.error = null;
			this.renaming_lang[lang.id] = true;
			// Disable Rename/Delete buttons.
			document.getElementById("rename-" + lang.id).disabled = true;
			document.getElementById("delete-" + lang.id).disabled = true;
			// Next tick, after the input has been enabled, select it.
			this.$nextTick(() => {
				document.getElementById("lang-input-" + lang.id).select();
			});
		},
		save_renaming(lang) {
			this.error = null;
			this.renaming_lang[lang.id] = false;
			// Reenable Rename/Delete buttons.
			document.getElementById("rename-" + lang.id).disabled = false;
			document.getElementById("delete-" + lang.id).disabled = false;
			// Read the new name and send it to the server.
			let input = document.getElementById("lang-input-" + lang.id);
			this.put_lang(lang.id, input.value);
			// Deselect.
			window.getSelection().removeAllRanges();
		},
		cancel_renaming(lang) {
			this.error = null;
			this.renaming_lang[lang.id] = false;
			// Reenable Rename/Delete buttons.
			document.getElementById("rename-" + lang.id).disabled = false;
			document.getElementById("delete-" + lang.id).disabled = false;
			// Reset the language text input to its original value and deselect it.
			let input = document.getElementById("lang-input-" + lang.id);
			input.value = lang.name;
			// Deselect.
			window.getSelection().removeAllRanges();
		},
		post_lang() {
			// Send the request.
			const url = "/lang/" + this.$refs.lang_input.value;
			const options = {
				method: "POST",
				headers: { "Content-Type": "application/text" },
			};
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.$refs.lang_input.value = "";
					this.update_langs();
				} else {
					this.error = "Invalid name."
				}
				this.$refs.lang_input.select();
			});
		},
		put_lang(id, name) {
			const url = "/lang/" + id + "/" + name;
			const options = {
				method: "PUT",
				headers: { "Content-Type": "application/text" },
			};
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.update_langs();
				} else {
					this.error = "Invalid name."
				}
			});
		},
		delete_lang(id) {
			// Send the request.
			const url = "/lang/" + id;
			const options = {
				method: "DELETE",
				headers: { "Content-Type": "application/text" },
			};
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.update_langs();
				} else {
					this.error = "Could not delete lang";
				}
			});
		},
		// Word classes
		update_classes() {
			fetch("/classes/English").then(response => {
				response.json().then(word_classes => {
					this.word_classes = word_classes;
				});
			});
		},
		post_class() {
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
	},
	mounted() {
		this.update_langs();
		this.$refs.lang_input.focus();
	}
};

Vue.createApp(app).mount("#app");
