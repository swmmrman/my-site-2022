use std::path::Path;
use rocket::form::Form;
use rocket::{fs::FileServer, response::content::RawHtml};
#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct FormFeilds<'l> {
    text_field: &'l str,
    #[field(default="Empty String")]
    other_text: String,
    optional: i32,
}

#[get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(Path::new("public_html/index.html")).await.ok()
}

#[post("/post.html", data = "<fields>")]
async fn post(fields: Form<FormFeilds<'_>>) -> RawHtml<String> {
    let text = format!(r#"<html>{}<br>{}<br>{}</html>"#, 
                                fields.text_field,
                                fields.other_text,
                                fields.optional,
    ).to_owned();
    RawHtml(
        text
    )
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
}
