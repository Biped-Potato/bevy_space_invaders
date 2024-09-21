use bevy::prelude::*;

use crate::alien;
use crate::resolution;
use crate::player;
use crate::projectile;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                (
                    alien::AlienPlugin,
                    resolution::ResolutionPlugin,
                    player::PlayerPlugin,
                    projectile::ProjectilePlugin,
                )
            )
            .add_systems(Startup, setup_scene);
    }
}
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}
