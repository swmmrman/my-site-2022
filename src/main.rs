use std::path::{PathBuf, Path};
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
async fn index() -> Result<RawHtml<String>, rocket::http::Status> {
    my_site_2022::make_page(PathBuf::new().join("index.html"), false)
}

#[get("/index.html")]
async fn index_redirect() -> rocket::response::Redirect {
    rocket::response::Redirect::to(rocket::uri!("/"))
}

#[get("/admin")]
async fn admin_index() -> Result<RawHtml<String>, rocket::http::Status> {
    my_site_2022::make_page(PathBuf::new().join("index.html"), true)
}

#[get("/admin/<page>")]
async fn get_admin_page(page: PathBuf) -> Result<RawHtml<String>, rocket::http::Status> {
    my_site_2022::make_page(page, true)
}

#[get("/<page..>")]
async fn get_page(page: PathBuf) -> Result<RawHtml<String>, rocket::http::Status> {
    my_site_2022::make_page(page, false)
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
async fn four_oh_four() -> Result<RawHtml<String>, rocket::http::Status> {
    let (page, title) = my_site_2022::get_page_title("errors/404.html");
    let tmpl = std::fs::read_to_string("template/main.tmpl.html").unwrap();
    let output = tmpl.replace("[content]", &page);
    Ok(RawHtml(tmpl.replace("[title]", &title)))
}

#[catch(403)]
async fn four_oh_three() -> Result<RawHtml<String>, rocket::http::Status> {
    let (page, title) = my_site_2022::get_page_title("errors/403.html");
    let tmpl = std::fs::read_to_string(Path::new("template/admin/main.tmpl.html")).unwrap();
    let output = tmpl.replace("[content]", &page);
    Ok(RawHtml(output.replace("[title]", &title)))
}

#[catch(404)]
async fn four_oh_four_admin() -> Result<RawHtml<String>, rocket::http::Status> {
    let (page, title) = my_site_2022::get_page_title("errors/404a.html");
    let tmpl = std::fs::read_to_string(Path::new("template/admin/main.tmpl.html")).unwrap();
    let output = tmpl.replace("[content]", &page);
    Ok(RawHtml(output.replace("[title]", &title)))
}
#[catch(418)]
async fn teapot() -> Result<RawHtml<String>, rocket::http::Status> {
    let (page, title) = my_site_2022::get_page_title("errors/418.html");
    let mut tmpl = std::fs::read_to_string(Path::new("template/main.tmpl.html")).unwrap();
    tmpl = tmpl.replace("[content]", &page);
    Ok(RawHtml(tmpl.replace("[title]", &title)))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post, get_page, index_redirect, admin_index, get_admin_page])
        .mount("/js/", FileServer::from("public_html/js/").rank(-2))
        .mount("/css/", FileServer::from("public_html/css/").rank(-2))
        .mount("/images/", FileServer::from("public_html/images/").rank(-2))
        .register("/", catchers![four_oh_four, four_oh_three, teapot])
        .register("/admin", catchers![four_oh_four_admin])
}
