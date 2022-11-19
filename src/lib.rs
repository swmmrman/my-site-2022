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