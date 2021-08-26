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
};
use rocket_sync_db_pools::{
	database,
	rusqlite::{params, Connection},
};

fn map_err<E: std::error::Error>(err: E) -> Status {
	warn!("{}", err.to_string());
	Status::InternalServerError
}

#[get("/")]
async fn index(conn: DbConn) -> Result<Redirect, Status> {
	// Initialize database.
	conn.run(|c| {
		c.execute_batch(include_str!("create.sql"))
			.map_err(map_err)?;
		// Redirect to the app page.
		Ok(Redirect::to("app.html"))
	})
	.await
}

#[derive(Serialize, Deserialize)]
struct LangIn {
	name: String,
}

#[post("/langs", format = "json", data = "<lang>")]
async fn create_lang(lang: Json<LangIn>, conn: DbConn) -> Result<(), Status> {
	conn.run(move |c| {
		let query = "insert into lang (name) values (?1);";
		c.execute(query, params![lang.name]).map_err(map_err)?;
		Ok(())
	})
	.await
}

#[put("/langs/<id>", format = "json", data = "<lang>")]
async fn edit_lang(id: i64, lang: Json<LangIn>, conn: DbConn) -> Result<(), Status> {
	conn.run(move |c| {
		let query = "update lang set name = ?2 where id = ?1;";
		c.execute(query, params![id, lang.name]).map_err(map_err)?;
		Ok(())
	})
	.await
}

#[derive(Serialize, Deserialize)]
struct LangOut {
	id: i64,
	name: String,
}

#[get("/langs")]
async fn get_langs(conn: DbConn) -> Result<Json<Vec<LangOut>>, Status> {
	conn.run(|c| {
		let query = "select id, name from lang order by name collate nocase;";
		let mut statement = c.prepare(query).map_err(map_err)?;
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
	})
	.await
}

#[delete("/langs/<id>")]
async fn delete_lang(id: i64, conn: DbConn) -> Result<(), Status> {
	conn.run(move |c| {
		let query = "delete from lang where id = $1;";
		let mut statement = c.prepare(query).map_err(map_err)?;
		statement.execute(params![id]).map_err(map_err)?;
		Ok(())
	})
	.await
}

#[derive(Serialize, Deserialize)]
struct WordClassIn {
	lang_id: i64,
	name: String,
}

#[post("/word-classes", format = "json", data = "<word_class>")]
async fn create_word_class(word_class: Json<WordClassIn>, conn: DbConn) -> Result<(), Status> {
	conn.run(|c| {
		let query = "insert into word_class (lang_id, name) values (?1, ?2);";
		let word_class = word_class.into_inner();
		c.execute(query, params![word_class.lang_id, word_class.name])
			.map_err(map_err)?;
		Ok(())
	})
	.await
}

#[put("/word-classes/<id>", format = "json", data = "<word_class>")]
async fn edit_word_class(
	id: i64,
	word_class: Json<WordClassIn>,
	conn: DbConn,
) -> Result<(), Status> {
	conn.run(move |c| {
		let query = "update word_class set lang_id = ?2, name = ?3 where id = ?1;";
		c.execute(query, params![id, word_class.lang_id, word_class.name])
			.map_err(map_err)?;
		Ok(())
	})
	.await
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
	conn: DbConn,
) -> Result<Json<Vec<WordClassOut>>, Status> {
	conn.run(move |c| {
		let query = concat!(
			"select id, lang_id, name from word_class ",
			"where ?1 is null or lang_id = ?1 ",
			"order by name collate nocase;",
		);
		let mut statement = c.prepare(query).map_err(map_err)?;
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
	})
	.await
}

#[delete("/word-classes/<id>")]
async fn delete_word_class(id: i64, conn: DbConn) -> Result<(), Status> {
	conn.run(move |c| {
		let query = "delete from word_class where id = $1;";
		let mut statement = c.prepare(query).map_err(map_err)?;
		statement.execute(params![id]).map_err(map_err)?;
		Ok(())
	})
	.await
}

#[database("translatabase")]
struct DbConn(Connection);

#[launch]
fn rocket() -> _ {
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
		.attach(DbConn::fairing())
}
