use sycamore::prelude::*;
use sycamore_router::Route;

pub mod dashboard;

use dashboard::page::Dashboard;
use sycamore_web::tags::view;

use crate::app::context::AppState;

#[derive(Route, PartialEq, Debug, Clone, Copy)]
pub enum FeatureRoutes {
    #[to("")]
    Main,
    #[not_found]
    NotFound,
}

pub fn register_routes(route: FeatureRoutes, state: AppState) -> View {
    match route {
        FeatureRoutes::Main => {
            if state.is_user_logged_in().clone() {
                view! {
                   Dashboard() {}
                }
            } else {
                view! {
                    h1 { "Please log in to access the dashboard." }
                }
            }
        }
        FeatureRoutes::NotFound => {
            view! {
                h1 { "404 Not Found" }
            }
        }
    }
}