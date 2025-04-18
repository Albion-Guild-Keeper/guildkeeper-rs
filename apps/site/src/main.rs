use app::router::App;

mod app;
mod features;
mod services;
mod errors;
mod models;

fn main() {
    sycamore::render(App);
    tracing::info!("Starting application...");
}