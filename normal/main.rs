#![forbid(unsafe_code)]

mod app;
mod behaviour;
mod components;
mod icons;
mod markdown;
mod posts;
mod theme;

#[tokio::main]
async fn main() {
    topcoat::start(app::router())
        .await
        .expect("the topcoat server should start");
}
