#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, NamedFile};

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open("./WebPages/404.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./WebPages"))
        .register("/", catchers![not_found])
}