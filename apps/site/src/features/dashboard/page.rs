use sycamore::prelude::*;
use sycamore_futures::spawn_local;
use sycamore_rstml::html;

use crate::services::dashboard::calls::get_guildslist;

#[component]
pub fn Dashboard() -> View {
    let response = create_signal("Loading...".to_string());

    spawn_local(async move {
        let result = get_guildslist().await;
        match result {
            Ok(guilds_list_response) => {
                response.set(format!("{:?}", guilds_list_response.guilds));
            }
            Err(err) => {
                response.set(format!("Error: {}", err));
            }
        }
    });

    view!(
        div(class="container") {
            h1 { "Dashboard" }
            p { (response) }
        }
    )
}