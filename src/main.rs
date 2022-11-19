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
    let mut title = std::fs::read_to_string(Path::new("pages/index.html")).unwrap();
    let index_content = title.split_off(title.find("\n").unwrap());
    let mut output = main_tmpl.replace("[content]", &index_content);
    output = output.replace("[title]", &title);
    Some(RawHtml(output))
}

#[get("/index.html")]
async fn index_redirect() -> rocket::response::Redirect {
    rocket::response::Redirect::to(rocket::uri!("/"))
}

#[get("/admin")]
async fn admin_index() -> Option<RawHtml<String>> {
    let admin_tmpl = std::fs::read_to_string(Path::new("template/admin/main.tmpl.html")).unwrap();
    let index_content = std::fs::read_to_string(Path::new("pages/admin/index.html")).unwrap();
    let mut output = admin_tmpl.replace("[content]", &index_content);
    output = output.replace("[title]", "Admin Pages");
    Some(RawHtml(output))
}

#[get("/admin/<page>")]
async fn get_admin_page(page: &str) -> Result<RawHtml<String>, rocket::http::Status> {
    my_site_2022::make_page(page, true)
}

#[get("/<page>")]
async fn get_page(page: &str) -> Result<RawHtml<String>, rocket::http::Status> {
    my_site_2022::make_page(&page, false)
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
        .mount("/", routes![index, post, get_page, index_redirect, admin_index, get_admin_page])
        .mount("/js/", FileServer::from("public_html/js/"))
        .mount("/css/", FileServer::from("public_html/css/"))
        .register("/", catchers![four_oh_four, four_oh_three])
        .register("/admin", catchers![four_oh_four_admin])
}
