use std::path::Path;
use rocket::form::Form;
use rocket::{fs::FileServer, response::content::RawHtml};
#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct FormFeilds<'l> {
    text_field: &'l str,
    #[field(default = "Empty String")]
    other_text: String,
    optional: i32,
}

#[get("/")]
async fn index() -> Option<RawHtml<String>> {
    let main_tmpl = std::fs::read_to_string(Path::new("template/main.tmpl.html")).unwrap();
    Some(RawHtml(main_tmpl))
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

#[catch(404)]
async fn four_oh_four() -> rocket::fs::NamedFile {
    rocket::fs::NamedFile::open("errors/404.html").await.ok().unwrap()
}

#[catch(403)]
async fn four_oh_three() -> rocket::fs::NamedFile {
    rocket::fs::NamedFile::open("errors/403.html").await.ok().unwrap()
}

#[catch(404)]
async fn four_oh_four_admin() -> rocket::fs::NamedFile {
    rocket::fs::NamedFile::open("errors/404a.html").await.ok().unwrap()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
        .register("/", catchers![four_oh_four, four_oh_three])
        .register("/admin", catchers![four_oh_four_admin])
}
