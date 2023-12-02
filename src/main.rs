mod snake;
mod ball;
mod camera;
mod game_area;

use bevy::prelude::*;
use ball::BallPlugin;
use camera::CameraPlugin;
use game_area::GameAreaPlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins((  
                DefaultPlugins.set(WindowPlugin{
                    primary_window: Some(Window {
                        title: "Snong".into(),
                        resolution: (520., 520.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
                BallPlugin,
                CameraPlugin,
                GameAreaPlugin,
                SnakePlugin,
            ))
            .run();
}
