use std::fs;

fn main() {
    fs::remove_dir_all("dist").ok();
    fs::create_dir("dist").unwrap();
    fs::copy("static/index.html", "dist/index.html").unwrap();
    fs::copy("static/.htaccess", "dist/.htaccess").unwrap();
}
