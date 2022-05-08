#[macro_use]
extern crate diesel;
extern crate chrono;

mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;
mod server;

fn main() -> std::io::Result<()> {
    server::router::run()
}
