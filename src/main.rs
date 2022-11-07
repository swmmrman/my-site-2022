use std::path::Path;
use rocket::fs::FileServer;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/index.html")).await.ok()
}

#[get("/css/<file_name>")]
async fn css(file_name: &str) -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/css").join(file_name)).await.ok()
}

#[get("/js/<file_name>")]
async fn js(file_name: &str) -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/js").join(file_name)).await.ok()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, css])
        .mount("/js/", FileServer::from("public_html/js/"))
        
}
