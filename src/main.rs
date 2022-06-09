use rocket::fs::{relative, FileServer};
use rocket::*;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
