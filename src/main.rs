use rocket::fs::{FileServer, relative};
use std::env;

#[rocket::launch]
fn rocket() -> _ {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".into()) // fallback for local testing
        .parse()
        .expect("PORT must be a number");

    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .configure(rocket::Config {
            port,
            address: "0.0.0.0".parse().unwrap(),
            ..Default::default()
        })
}

// use rocket::fs::{FileServer, relative};

// #[rocket::launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", FileServer::from(relative!("static")))
// }