#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use tracing::{span, Level};
use rocket_dyn_templates::Template;
use rocket::fs::{NamedFile, relative};

use std::collections::HashMap;
use std::path::{PathBuf, Path};


#[get("/")]
async fn index() -> Template {
    trace!("Entering index()");
    let span = span!(Level::TRACE, "index_handler");
    let _ = span.enter();
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/css/<path..>")]
async fn serve_css(path: PathBuf) -> Option<NamedFile> {
    trace!("serve_css path: {:?}", path);

    let file = Path::new(relative!("css")).join(path);

    trace!("serve_css path: {:?}", file);
    NamedFile::open(file).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", routes![serve_css])
}
