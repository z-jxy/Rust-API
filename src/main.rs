#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

mod db;
//mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
}
