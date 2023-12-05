use bevy::{prelude::*, window::PrimaryWindow};

mod loading;
use loading::LoadingPlugin;

mod level;

mod unit;
use unit::{SpawnUnit, UnitPlugin};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, States, Default)]
enum AppState {
    #[default]
    Loading,
    Playing,
    // Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();

        app.add_plugins((LoadingPlugin, UnitPlugin));

        app.add_systems(OnEnter(AppState::Playing), spawn_level);

        app.add_systems(Update, spawn_player.run_if(in_state(AppState::Playing)));
    }
}

fn spawn_level(mut commands: Commands) {
    // TODO: set camera.projection.scaling_mode
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mouse_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut spawn_player_ev: EventWriter<SpawnUnit>,
) {
    if mouse_input.just_released(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();
        let window = window.single();
        if let Some(pos) = window
            .cursor_position()
            .and_then(|c| camera.viewport_to_world(camera_transform, c))
            .map(|ray| ray.origin.truncate())
        {
            println!("{pos:?}");
            spawn_player_ev.send(SpawnUnit::at(pos));
        }
    }
}
