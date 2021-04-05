#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use log::warn;
use rocket::{
	http::Status,
	response::Redirect,
	State,
};
use rocket_contrib::{
	json::Json,
	serve::StaticFiles,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use webbrowser;

use std::sync::Mutex;

fn map_err<E: std::error::Error>(err: E) -> Status {
	warn!("{}", err.to_string());
	Status::InternalServerError
}

#[get("/")]
fn index(conn: State<Mutex<Connection>>) -> Result<Redirect, Status> {
	// Initialize database.
	conn.lock()
		.map_err(map_err)?
		.execute_batch(include_str!("create.sql"))
		.map_err(map_err)?;
	// Redirect to the app page.
	Ok(Redirect::to("app.html"))
}

#[derive(Serialize, Deserialize)]
struct Lang {
	id: i64,
	name: String,
}

#[post("/langs", format = "json", data = "<name>")]
fn create_lang(name: Json<String>, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().map_err(map_err)?;
	conn.execute("insert into lang (name) values (?1)", params![name.as_str()]).map_err(map_err)?;
	Ok(())
}

#[get("/langs")]
fn get_langs(conn: State<Mutex<Connection>>) -> Result<Json<Vec<Lang>>, Status> {
	let conn = conn.lock().map_err(map_err)?;
	let mut statement = conn.prepare("
		select id, name from lang
		order by name collate nocase
	").map_err(map_err)?;
	let rows = statement.query_map(params![], |row| {
		Ok(Lang {
			id: row.get(0)?,
			name: row.get(1)?,
		})
	}).map_err(map_err)?;
	let mut langs = Vec::new();
	for lang in rows {
		langs.push(lang.map_err(map_err)?);
	}
	Ok(Json(langs))
}

#[put("/langs/<id>", format = "json", data = "<name>")]
fn edit_lang(id: i64, name: Json<String>, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().map_err(map_err)?;
	conn.execute("
		update lang
		set name = ?2
		where id = ?1
	", params![id, name.as_str()]).map_err(map_err)?;
	Ok(())
}

#[delete("/langs/<id>")]
fn delete_lang(id: i64, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().map_err(map_err)?;
	let mut statement = conn.prepare("
		delete from lang
		where id = $1
	").map_err(map_err)?;
	statement.execute(params![id]).map_err(map_err)?;
	Ok(())
}

#[derive(Serialize, Deserialize)]
struct WordClass {
	id: i64,
	name: String,
}

#[post("/classes/<lang_id>", format = "json", data = "<name>")]
fn create_class(lang_id: i64, name: Json<String>, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().map_err(map_err)?;
	conn.execute(
		"insert into class (lang_id, name) values (?1, ?2)",
		params![lang_id, name.as_str()],
	).map_err(map_err)?;
	Ok(())
}

#[get("/classes/<lang_id>")]
fn get_classes(lang_id: i64, conn: State<Mutex<Connection>>) -> Result<Json<Vec<WordClass>>, Status> {
	let conn = conn.lock().map_err(map_err)?;
	let mut statement = conn.prepare("
		select id, name from class
		where lang_id = ?1
		order by name collate nocase
	").map_err(map_err)?;
	let word_classes = statement.query_map(params![lang_id], |row| {
		Ok(WordClass {
			id: row.get(0)?,
			name: row.get(1)?,
		})
	}).map_err(map_err)?;
	let mut word_class_names = Vec::new();
	for word_class in word_classes {
		word_class_names.push(word_class.map_err(map_err)?);
	}
	Ok(Json(word_class_names))
}

fn main() {
	// Open database connection.
	let connection = Connection::open("translatabase.db")
		.expect("Could not open database connection");
	// Launch web browser.
	webbrowser::open("http://localhost:8000/")
		.expect("Could not open web browser");
	// Launch server.
	rocket::ignite()
		.mount("/", routes![
			index,
			// Languages
			create_lang,
			get_langs,
			edit_lang,
			delete_lang,
			// Classes
			get_classes,
			create_class,
		])
		.mount("/", StaticFiles::from("www"))
		.manage(Mutex::new(connection))
		.launch();
}
