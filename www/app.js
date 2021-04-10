"use strict";

const app = {
	data() {
		return {
			panel: "langs",
			langs: [],
			renaming_lang: {},
			word_classes: [],
			renaming_word_class: {},
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
		start_renaming_lang(lang) {
			this.error = null;
			this.renaming_lang[lang.id] = true;
			// Disable Rename/Delete buttons.
			document.getElementById("rename-lang-" + lang.id).disabled = true;
			document.getElementById("delete-lang-" + lang.id).disabled = true;
			// Next tick, after the input has been enabled, select it.
			this.$nextTick(() => {
				document.getElementById("lang-input-" + lang.id).select();
			});
		},
		save_renaming_lang(lang) {
			this.error = null;
			this.renaming_lang[lang.id] = false;
			// Reenable Rename/Delete buttons.
			document.getElementById("rename-lang-" + lang.id).disabled = false;
			document.getElementById("delete-lang-" + lang.id).disabled = false;
			// Read the new name and send it to the server.
			let input = document.getElementById("lang-input-" + lang.id);
			this.put_lang(lang.id, input.value);
			// Deselect.
			window.getSelection().removeAllRanges();
		},
		cancel_renaming_lang(lang) {
			this.error = null;
			this.renaming_lang[lang.id] = false;
			// Reenable Rename/Delete buttons.
			document.getElementById("rename-lang-" + lang.id).disabled = false;
			document.getElementById("delete-lang-" + lang.id).disabled = false;
			// Reset the language text input to its original value and deselect it.
			let input = document.getElementById("lang-input-" + lang.id);
			input.value = lang.name;
			// Deselect.
			window.getSelection().removeAllRanges();
		},
		post_lang() {
			const options = {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({
					"name": this.$refs.lang_input.value,
				}),
			};
			fetch("/langs", options).then(response => {
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
			const url = "/langs/" + encodeURIComponent(id);
			const options = {
				method: "PUT",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({
					"name": name,
				}),
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
			const url = "/langs/" + encodeURIComponent(id);
			const options = {
				method: "DELETE",
				headers: { "Content-Type": "application/text" },
			};
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.update_langs();
				} else {
					this.error = "Could not delete language";
				}
			});
		},
		// Word classes
		update_word_classes() {
			const url = "/word-classes?lang_id=" + encodeURIComponent(this.$refs.lang_select.value);
			fetch(url).then(response => {
				response.json().then(word_classes => {
					this.word_classes = word_classes;
				});
			});
		},
		start_renaming_word_class(word_class) {
			this.error = null;
			this.renaming_word_class[word_class.id] = true;
			// Disable Rename/Delete buttons.
			document.getElementById("rename-word-class-" + word_class.id).disabled = true;
			document.getElementById("delete-word-class-" + word_class.id).disabled = true;
			// Next tick, after the input has been enabled, select it.
			this.$nextTick(() => {
				document.getElementById("word-class-input-" + word_class.id).select();
			});
		},
		save_renaming_word_class(word_class) {
			this.error = null;
			this.renaming_word_class[word_class.id] = false;
			// Reenable Rename/Delete buttons.
			document.getElementById("rename-word-class-" + word_class.id).disabled = false;
			document.getElementById("delete-word-class-" + word_class.id).disabled = false;
			// Read the new name and send it to the server.
			const lang_id = parseInt(this.$refs.lang_select.value);
			const name = document.getElementById("word-class-input-" + word_class.id).value;
			this.put_word_class(word_class.id, lang_id, name);
			// Deselect.
			window.getSelection().removeAllRanges();
		},
		cancel_renaming_word_class(word_class) {
			this.error = null;
			this.renaming_word_class[word_class.id] = false;
			// Reenable Rename/Delete buttons.
			document.getElementById("rename-word-class-" + word_class.id).disabled = false;
			document.getElementById("delete-word-class-" + word_class.id).disabled = false;
			// Reset the language text input to its original value and deselect it.
			let input = document.getElementById("word-class-input-" + word_class.id);
			input.value = word_class.name;
			// Deselect.
			window.getSelection().removeAllRanges();
		},
		post_word_class() {
			const url = "/word-classes/";
			const options = {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({
					"name": this.$refs.word_class_input.value,
					"lang_id": parseInt(this.$refs.lang_select.value),
				}),
			}
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.$refs.word_class_input.value = "";
				} else {
					this.error = "Invalid word class";
				}
				this.update_word_classes();
				this.$refs.word_class_input.select();
			});
		},
		put_word_class(id, lang_id, name) {
			const url = "/word-classes/" + encodeURIComponent(id);
			const options = {
				method: "PUT",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({
					"lang_id": lang_id,
					"name": name,
				}),
			};
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.update_word_classes();
				} else {
					this.error = "Invalid word class."
				}
			});
		},
		delete_word_class(id) {
			const url = "/word-classes/" + encodeURIComponent(id);
			const options = {
				method: "DELETE",
				headers: { "Content-Type": "application/text" },
			};
			fetch(url, options).then(response => {
				if (response.status == 200) {
					this.error = null;
					this.update_word_classes();
				} else {
					this.error = "Could not delete word class";
				}
			});
		},
	},
	mounted() {
		this.update_langs();
		this.$refs.lang_input.focus();
	}
};

Vue.createApp(app).mount("#app");
