use rocket::fs::{relative, FileServer};
use rocket::serde::Serialize;
use rocket::*;
use rocket_dyn_templates::{context, Template};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NavLink {
    display: String,
    path: String,
}

impl NavLink {
    fn all() -> Vec<Self> {
        vec![
            Self {
                display: "Arcana Zero".to_string(),
                path: "/".to_string(),
            },
            Self {
                display: "Games".to_string(),
                path: "/games".to_string(),
            },
            Self {
                display: "Blog".to_string(),
                path: "/blog".to_string(),
            },
        ]
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    title: String,
    nav_links: Vec<NavLink>,
}

impl Context {
    fn new(title: String) -> Self {
        Self {
            title,
            nav_links: NavLink::all(),
        }
    }
}

#[get("/")]
fn index() -> Template {
    Template::render("index", Context::new("".to_string()))
}

#[get("/blog")]
fn blog() -> Template {
    Template::render("blog", Context::new("Blog".to_string()))
}

#[get("/games")]
fn games() -> Template {
    Template::render("games", Context::new("Games".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, games, blog])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
