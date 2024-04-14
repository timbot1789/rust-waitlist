#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

mod waitlist_entry;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::Serialize;
use rocket::form::Form;
use rocket::fs::{FileServer, relative};

use rocket_dyn_templates::Template;
use waitlist_entry::WaitlistEntry;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    flash: Option<(String, String)>,
    waitlist: Vec<WaitlistEntry>
}

impl Context {
    pub async fn err<M: std::fmt::Display>(conn: &DbConn, msg: M) -> Context {
        Context {
            flash: Some(("error".into(), msg.to_string())),
            waitlist: WaitlistEntry::all(conn).await.unwrap_or_default()
        }
    }

    pub async fn raw(conn: &DbConn, flash: Option<(String, String)>) -> Context {
        match WaitlistEntry::all(conn).await {
            Ok(waitlist) => Context { flash, waitlist},
            Err(e) => {
                error_!("DB Task::all() error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    waitlist: vec![]
                }
            }
        }
    }
}

#[post("/", data = "<waitlist_form>")]
async fn new(waitlist_form: Form<WaitlistEntry>, conn: DbConn) -> Flash<Redirect> {
    let waitlist = waitlist_form.into_inner();
    if waitlist.email.is_empty() {
        Flash::error(Redirect::to("/"), "Email cannot be empty")
    } else if let Err(e) = WaitlistEntry::insert(waitlist, &conn).await {
        error_!("DB insertion error: {}", e);
        Flash::error(Redirect::to("/"), "Entry could not be inserted due an internal error.")
    } else {
        Flash::success(Redirect::to("/"), "Successfully added to waitlist")
    }
}

#[delete("/<email>")]
async fn delete(email: String, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    match WaitlistEntry::delete_with_email(email.clone(), &conn).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Entry was deleted.")),
        Err(e) => {
            error_!("DB deletion({}) error: {}", email, e);
            Err(Template::render("index", Context::err(&conn, "Failed to delete entry").await))
        }
    }
}

#[get("/")]
async fn index(flash: Option<FlashMessage<'_>>, conn: DbConn) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("index", Context::raw(&conn, flash).await)
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .mount("/", routes![delete])
        .mount("/", routes![new])
}
