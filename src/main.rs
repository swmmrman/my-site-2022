use std::io::{Error, ErrorKind};
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
    let index_content = std::fs::read_to_string(Path::new("pages/index.html")).unwrap();
    let mut output = main_tmpl.replace("[content]", &index_content);
    output = output.replace("[title]", "Home Page");
    Some(RawHtml(output))
}

#[get("/index.html")]
async fn index_redirect() -> rocket::response::Redirect {
    rocket::response::Redirect::to(rocket::uri!("/"))
}

#[get("/<page>")]
async fn get_page(page: &str) -> Result<RawHtml<String>, rocket::http::Status> {
    let main_tmpl = std::fs::read_to_string(Path::new("template/main.tmpl.html")).unwrap();
    let page_results = std::fs::read_to_string(Path::new("pages/").join(page));
    let mut page_content = String::new();
    match page_results {
        Ok(p) => page_content.push_str(&p),
        Err(e) => return Err(parse_error(e)),
    }
    let output = main_tmpl.replace("[content]", &page_content);

    Ok(RawHtml(output))
}

fn parse_error(e: Error) -> rocket::http::Status {
    match e.kind() {
        ErrorKind::NotFound => rocket::http::Status::NotFound,
        ErrorKind::PermissionDenied => rocket::http::Status::Forbidden,
        _ => rocket::http::Status::ImATeapot,
    }
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
        .mount("/", routes![index, post, get_page, index_redirect])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
        .register("/", catchers![four_oh_four, four_oh_three])
        .register("/admin", catchers![four_oh_four_admin])
}
