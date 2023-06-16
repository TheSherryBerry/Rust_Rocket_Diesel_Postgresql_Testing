#[macro_use] extern crate rocket;

//   http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_all() -> &'static str {
    "Todo Get All"
}




// add more endpoints here

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
        .mount("/", routes![get_all])
}
