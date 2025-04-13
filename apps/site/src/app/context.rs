// ? @todo Context App State Management ????
use sycamore::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    // Example state: Add the fields you need
    is_logged: Signal<bool>,
    // pub user_name: Signal<String>, // Example of another field
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_logged: create_signal(false),
        }
    }
    pub fn toggle_login(&self) {
        self.is_logged.set(!self.is_logged.get());
    }
    pub fn set_login(&self, logged_in: bool) {
        self.is_logged.set(logged_in);
    }
    pub fn is_user_logged_in(&self) -> bool {
        self.is_logged.get()
    }
}