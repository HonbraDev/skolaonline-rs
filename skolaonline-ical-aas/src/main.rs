mod endpoint;
mod error;

#[macro_use]
extern crate rocket;

#[catch(404)]
fn not_found() -> &'static str {
    "The requested resource was not found."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                endpoint::index,
                endpoint::calendar,
                endpoint::calendar_browser,
                endpoint::calendar_not_acceptable,
            ],
        )
        .register("/", catchers![not_found])
}
