use std::path::Path;
use std::io::{Error, ErrorKind};
use rocket::response::content::RawHtml;

pub fn sing_99_bottles() -> String {
    let mut out = String::new();
    for i in (3..=99).rev() {
        out.push_str(
            &format!("{} bottles of beer on the wall, {} bottles of beer, take one down pass it around, {} bottles of beer on the wall.<br>",
                    i, i, (i-1) )
        );
    }
    out.push_str(
        "2 bottles of beer on the wall, 2 bottles of beer, take one down, pass it around,  1 bottle of beer on the wall.<br>"
    );
    out.push_str(
        "1 bottle of beer on the wall, 1 bottle of beer, take it down, pass it around, All out.<br>"
    );
    out
}

pub fn make_page(page: &str) -> Result<RawHtml<String>, rocket::http::Status> {
    let main_tmpl = std::fs::read_to_string(Path::new("template/main.tmpl.html")).unwrap();
    if page == "99-bottles.html" {
        return Ok(RawHtml(main_tmpl.replace("[content]", &sing_99_bottles())));
    }
    let page_results = std::fs::read_to_string(Path::new("pages/").join(page));
    let mut title = String::new();
    match page_results {
        Ok(p) => title.push_str(&p),
        Err(e) => return Err(crate::parse_error(e)),
    }
    let page_content = title.split_off(title.find("\n").unwrap());
    let mut output = main_tmpl.replace("[content]", &page_content);
    output = output.replace("[title]", &title);
    Ok(RawHtml(output))
}

pub fn parse_error(e: Error) -> rocket::http::Status {
    match e.kind() {
        ErrorKind::NotFound => rocket::http::Status::NotFound,
        ErrorKind::PermissionDenied => rocket::http::Status::Forbidden,
        _ => rocket::http::Status::ImATeapot,
    }
}