use bevy::prelude::*;

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoundingBox{
            left_side: -260.,
            right_side: 260.,
            top_side: -260.,
            bottom_side: 260.,
        });
    }
}

#[derive(Resource, Debug)]
pub struct BoundingBox {
    pub left_side: f32,
    pub right_side: f32,
    pub top_side: f32,
    pub bottom_side: f32,
}