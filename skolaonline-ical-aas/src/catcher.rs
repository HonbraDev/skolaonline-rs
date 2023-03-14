use rocket::Catcher;

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[catch(404)]
fn not_found() -> &'static str {
    "The requested resource could not be found."
}
