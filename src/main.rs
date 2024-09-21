use bevy::prelude::*;

//link our modules to our project
pub mod game;
pub mod alien;
pub mod resolution;
pub mod player;
pub mod projectile;
fn main() {
    App::new()
        .add_plugins(
            (
                //list of plugins added to the game
                DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),

                game::GamePlugin,
            ),
            
        )
        .run();
}
