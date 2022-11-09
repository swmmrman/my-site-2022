use std::path::Path;
use rocket::fs::FileServer;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/index.html")).await.ok()
}
#[post("/post.html", data = "<text_field>")]
async fn post(text_field: &str) -> String {
    let text_fields: Vec<&str> = text_field.split("&").collect();
    let field_0: &str = text_fields[0].split("=").collect::<Vec<&str>>()[1];
    let field_1: &str = text_fields[1].split("=").collect::<Vec<&str>>()[0];
    format!("{}\n{}",
        field_0,
        field_1
    )
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
}
