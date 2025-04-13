use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::{app::context::AppState, features::{self, FeatureRoutes}};

#[derive(Route, Debug, PartialEq, Clone, Copy)]
pub enum AppRoutes {
    #[to("/<_..>")]
    Main(FeatureRoutes),
    #[not_found]
    NotFound,
}

pub fn App() -> View {
    let router = HistoryIntegration::new();
    let app_state = AppState::new();
    view! {
        Router(
            integration = router,
            view = move |route: ReadSignal<AppRoutes>| {
                match route.get() {
                    AppRoutes::Main(route) => features::register_routes(route, app_state),
                    AppRoutes::NotFound => view! { h1 {
                        "404 Not Found"
                    } },
                }
            }
        )
    }
}
