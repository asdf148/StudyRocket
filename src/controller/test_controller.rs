#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
pub fn hello_name(name: &str) -> String {
    format!("Hello, {}", name)
}