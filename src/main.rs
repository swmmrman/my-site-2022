use std::path::Path;
use rocket::fs::FileServer;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/index.html")).await.ok()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
}
