pub mod login;
pub mod index;
use crate::AppState;
#[allow(dead_code)]
/// get app state
pub fn get_app_state<'a>(state: &'a AppState) -> &'a AppState {
    state
}