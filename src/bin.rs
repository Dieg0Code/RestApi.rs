#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;


fn main() {
    rocket().launch();
}

#[get("/")]

fn get_movies() -> Json<String> {
    Json(String::from("Movies"))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/movies",
        routes![get_movies],
    )
}
