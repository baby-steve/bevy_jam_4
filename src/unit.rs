//! Module containg player related logic.
//! 
use bevy::prelude::*;

use crate::AppState;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnUnit>();

        app.add_systems(Update, handle_spawn_unit_event.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event)]
pub struct SpawnUnit {
    location: Vec2,
}

impl SpawnUnit {
    pub fn at(location: Vec2) -> Self {
        Self { location }
    }
}

fn handle_spawn_unit_event(mut commands: Commands, mut spawn_events: EventReader<SpawnUnit>) {
    for SpawnUnit { location } in spawn_events.read() {
        commands.spawn(SpriteBundle {
            sprite: Sprite { color: Color::PURPLE, custom_size: Some(Vec2::new(32., 32.)), ..default() },
            transform: Transform {
                translation: location.extend(1.0),
                ..default()
            },
            ..default()
        });
    }
}
