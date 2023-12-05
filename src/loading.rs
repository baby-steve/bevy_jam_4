//! Module containing logic related to loading.
//! 

use bevy::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_loading_complete.run_if(in_state(AppState::Loading)));
    }
}

fn on_loading_complete(mut app_state: ResMut<NextState<AppState>>) {
    // TODO: transition to Menu
    app_state.set(AppState::Playing);
}
