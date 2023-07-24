#[macro_use] extern crate rocket;

#[get("/")]
fn get() -> String {
    format!("Hello!")
}

#[post("/", data = "<data>")]
fn post(data: &str) -> String {
    format!("{}", data)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get,
        post
    ])
}
