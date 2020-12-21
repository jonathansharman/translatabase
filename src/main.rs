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
		.execute_batch(include_str!("init.sql"))
		.map_err(map_err)?;
	// Redirect to the languages page.
	Ok(Redirect::to("langs.html"))
}

struct Lang(String);

#[get("/langs")]
fn get_langs(conn: State<Mutex<Connection>>) -> Result<Json<Vec<String>>, Status> {
	let conn = conn.lock().map_err(map_err)?;
	let mut statement = conn.prepare("
		select * from langs
		order by name collate nocase
		").map_err(map_err)?;
	let langs = statement.query_map(params![], |row| {
		Ok(Lang(row.get(0).unwrap()))
	}).map_err(map_err)?;
	let mut lang_names = Vec::new();
	for lang in langs {
		lang_names.push(lang.unwrap().0);
	}
	Ok(Json(lang_names))
}

#[post("/lang/<name>")]
fn post_lang(name: String, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().map_err(map_err)?;
	conn.execute("insert into langs (name) values (?1)", params![name]).map_err(map_err)?;
	Ok(())
}

struct WordClass(String);

#[get("/classes/<lang>")]
fn get_classes(lang: String, conn: State<Mutex<Connection>>) -> Result<Json<Vec<String>>, Status> {
	let conn = conn.lock().map_err(map_err)?;
	let mut statement = conn.prepare("
		select name from classes
		where lang = ?1
		order by name collate nocase
	").map_err(map_err)?;
	let word_classes = statement.query_map(params![lang], |row| {
		Ok(WordClass(row.get(0).unwrap()))
	}).map_err(map_err)?;
	let mut word_class_names = Vec::new();
	for word_class in word_classes {
		word_class_names.push(word_class.unwrap().0);
	}
	Ok(Json(word_class_names))
}

#[post("/class/<lang>/<name>")]
fn post_class(lang: String, name: String, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().map_err(map_err)?;
	conn.execute("insert into classes (lang, name) values (?1, ?2)", params![lang, name]).map_err(map_err)?;
	Ok(())
}

fn main() {
	// Open database connection.
	let connection = Connection::open("translatabase.db").unwrap();
	// Launch web browser.
	webbrowser::open("http://localhost:8000/").unwrap();
	// Launch server.
	rocket::ignite()
		.mount("/", routes![
			index,
			get_langs,
			post_lang,
			get_classes,
			post_class,
		])
		.mount("/", StaticFiles::from("www"))
		.manage(Mutex::new(connection))
		.launch();
}
