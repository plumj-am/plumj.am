#![forbid(unsafe_code)]

mod app;
mod components;
mod home;
mod keys;
mod lines;

#[tokio::main]
async fn main() {
    topcoat::start(app::router())
        .await
        .expect("the topcoat server should start");
}
