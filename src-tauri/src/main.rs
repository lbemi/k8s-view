// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod boot;
mod router;
mod service;

use boot::server;
fn main() {
    server::run();
}
