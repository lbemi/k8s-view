// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod api;
mod boot;
mod error;
mod router;
mod service;
mod store;
mod utils;

use boot::server;
fn main() {
    server::run();
}
