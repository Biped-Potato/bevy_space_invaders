use bevy::{prelude::*, render::camera::ScalingMode};

mod alien;
mod player;
mod projectile;
mod resolution;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            alien::AlienPlugin,
            resolution::ResolutionPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
        ))
        .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands, resolution: Res<resolution::Resolution>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: ScalingMode::AutoMin {
                min_width: resolution.screen_dimensions.x,
                min_height: resolution.screen_dimensions.y,
            },
            ..default()
        },
        ..default()
    });
}
