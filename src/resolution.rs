use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin{
    fn build(&self, app: &mut App) {
        //prestartup runs before all of our in game startup functions
        app.add_systems(PreStartup,setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution{
    //pixel dimensions of our screen in the form of a 2d vector (width,height)
    pub screen_dimensions : Vec2,
    //the ratio of a pixel in our sprites to one on screen
    pub pixel_ratio : f32,
}
fn setup_resolution(mut commands : Commands,window_query : Query<&Window>)
{
    //query for window information
    let window = window_query.single();

    commands.insert_resource(Resolution{
        screen_dimensions : Vec2::new(window.width(),window.height()),
        pixel_ratio : 2.0,
    });
}