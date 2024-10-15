use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        //prestartup runs before all of our in game startup functions
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    //pixel dimensions of our screen in the form of a 2d vector (width,height)
    pub screen_dimensions: Vec2,
}

fn setup_resolution(mut commands: Commands) {
    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::splat(256.), //game field size (will scale to window size)
    });
}
