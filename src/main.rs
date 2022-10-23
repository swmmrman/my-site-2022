use std::path::Path;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile>{
    rocket::fs::NamedFile::open(Path::new("public_html/index.html")).await.ok()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
