#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

mod db;
//mod models;
pub mod schema;
//pub mod structs;
pub mod api_models;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
}
