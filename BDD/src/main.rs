mod api;
mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod utils;

use crate::config::load_config;
use crate::db::get_users;

fn main() {
    load_config();

    let users = get_users();
    println!("Users from fake DB: {:?}", users);

    println!("Server is running");
}