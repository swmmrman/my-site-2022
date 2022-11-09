use std::path::Path;
use rocket::fs::FileServer;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/index.html")).await.ok()
}
#[post("/post.html", data = "<text_field>")]
async fn post(text_field: &str) -> &str {
    text_field.clone()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
}
