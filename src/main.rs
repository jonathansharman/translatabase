use log::warn;
use rocket::{
	delete,
	fs::FileServer,
	get,
	http::Status,
	launch, post, put,
	response::Redirect,
	routes,
	serde::{json::Json, Deserialize, Serialize},
	State,
};
use rusqlite::{params, Connection};

use rocket::tokio::sync::Mutex;

fn map_err<E: std::error::Error>(err: E) -> Status {
	warn!("{}", err.to_string());
	Status::InternalServerError
}

#[get("/")]
async fn index(conn: State<Mutex<Connection>>) -> Result<Redirect, Status> {
	// Initialize database.
	conn.lock()
		.await
		.execute_batch(include_str!("create.sql"))
		.map_err(map_err)?;
	// Redirect to the app page.
	Ok(Redirect::to("app.html"))
}

#[derive(Serialize, Deserialize)]
struct LangIn {
	name: String,
}

#[post("/langs", format = "json", data = "<lang>")]
async fn create_lang(lang: Json<LangIn>, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().await;
	conn.execute("insert into lang (name) values (?1)", params![lang.name])
		.map_err(map_err)?;
	Ok(())
}

#[put("/langs/<id>", format = "json", data = "<lang>")]
async fn edit_lang(
	id: i64,
	lang: Json<LangIn>,
	conn: State<Mutex<Connection>>,
) -> Result<(), Status> {
	let conn = conn.lock().await;
	conn.execute(
		"
		update lang
		set name = ?2
		where id = ?1",
		params![id, lang.name],
	)
	.map_err(map_err)?;
	Ok(())
}

#[derive(Serialize, Deserialize)]
struct LangOut {
	id: i64,
	name: String,
}

#[get("/langs")]
async fn get_langs(conn: State<Mutex<Connection>>) -> Result<Json<Vec<LangOut>>, Status> {
	let conn = conn.lock().await;
	let mut statement = conn
		.prepare(
			"
		select id, name from lang
		order by name collate nocase",
		)
		.map_err(map_err)?;
	let lang_results = statement
		.query_map(params![], |row| {
			Ok(LangOut {
				id: row.get(0)?,
				name: row.get(1)?,
			})
		})
		.map_err(map_err)?;
	let mut langs = Vec::new();
	for lang_result in lang_results {
		langs.push(lang_result.map_err(map_err)?);
	}
	Ok(Json(langs))
}

#[delete("/langs/<id>")]
async fn delete_lang(id: i64, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().await;
	let mut statement = conn
		.prepare(
			"
			delete from lang
			where id = $1
			",
		)
		.map_err(map_err)?;
	statement.execute(params![id]).map_err(map_err)?;
	Ok(())
}

#[derive(Serialize, Deserialize)]
struct WordClassIn {
	lang_id: i64,
	name: String,
}

#[post("/word-classes", format = "json", data = "<word_class>")]
async fn create_word_class(
	word_class: Json<WordClassIn>,
	conn: State<Mutex<Connection>>,
) -> Result<(), Status> {
	let conn = conn.lock().await;
	let word_class = word_class.into_inner();
	conn.execute(
		"insert into word_class (lang_id, name) values (?1, ?2)",
		params![word_class.lang_id, word_class.name],
	)
	.map_err(map_err)?;
	Ok(())
}

#[put("/word-classes/<id>", format = "json", data = "<word_class>")]
async fn edit_word_class(
	id: i64,
	word_class: Json<WordClassIn>,
	conn: State<Mutex<Connection>>,
) -> Result<(), Status> {
	let conn = conn.lock().await;
	conn.execute(
		"
		update word_class
		set lang_id = ?2, name = ?3
		where id = ?1",
		params![id, word_class.lang_id, word_class.name],
	)
	.map_err(map_err)?;
	Ok(())
}

#[derive(Serialize, Deserialize)]
struct WordClassOut {
	id: i64,
	lang_id: i64,
	name: String,
}

#[get("/word-classes?<lang_id>")]
async fn get_word_classes(
	lang_id: Option<i64>,
	conn: State<Mutex<Connection>>,
) -> Result<Json<Vec<WordClassOut>>, Status> {
	let conn = conn.lock().await;
	let mut statement = conn
		.prepare(
			"
			select id, lang_id, name from word_class
			where ?1 is null or lang_id = ?1
			order by name collate nocase
			",
		)
		.map_err(map_err)?;
	let word_class_results = statement
		.query_map(params![lang_id], |row| {
			Ok(WordClassOut {
				id: row.get(0)?,
				lang_id: row.get(1)?,
				name: row.get(2)?,
			})
		})
		.map_err(map_err)?;
	let mut word_classes = Vec::new();
	for word_class_result in word_class_results {
		word_classes.push(word_class_result.map_err(map_err)?);
	}
	Ok(Json(word_classes))
}

#[delete("/word-classes/<id>")]
async fn delete_word_class(id: i64, conn: State<Mutex<Connection>>) -> Result<(), Status> {
	let conn = conn.lock().await;
	let mut statement = conn
		.prepare(
			"
			delete from word_class
			where id = $1
			",
		)
		.map_err(map_err)?;
	statement.execute(params![id]).map_err(map_err)?;
	Ok(())
}

#[launch]
fn rocket() -> _ {
	// Open database connection.
	let connection =
		Connection::open("translatabase.db").expect("Could not open database connection");
	// Launch web browser.
	webbrowser::open("http://localhost:8000/").expect("Could not open web browser");
	// Launch server.
	rocket::build()
		.mount(
			"/",
			routes![
				index,
				// Languages
				create_lang,
				get_langs,
				edit_lang,
				delete_lang,
				// Classes
				create_word_class,
				get_word_classes,
				edit_word_class,
				delete_word_class,
			],
		)
		.mount("/", FileServer::from("www"))
		.manage(Mutex::new(connection))
}
