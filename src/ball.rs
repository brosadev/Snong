use std::default;

use bevy::{
    core_pipeline::core_2d::graph::NAME,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::{game_area::BoundingBox, snake::SnakeHead};

const BALL_RADIUS: f32 = 10.;
const BALL_COLOR: Color = Color::WHITE;
const BALL_STARTING_POS: Vec3 = Vec3::ZERO;
const BALL_STARTING_SPEED: f32 = 300.;
const BALL_STARTING_DIR_VEC: Vec3 = Vec3::new(1., 2., 0.);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
        // app.add_systems(Update, ball_movement);
    }
}

#[derive(Component, Debug)]
pub struct Ball;

#[derive(Component, Debug)]
pub struct BallMovement {
    dir_vector: Vec3,
    speed: f32,
}

#[derive(Component, Debug)]
pub struct BallCollider {
    radius: f32,
}

#[derive(Bundle)]
struct BallBundle<M: Material2d> {
    tag: Ball,
    movement: BallMovement,
    collider: BallCollider,
    name: Name,
    material: MaterialMesh2dBundle<M>,
}

impl<M: Material2d> Default for BallBundle<M> {
    fn default() -> Self {
        Self {
            tag: Ball,
            movement: BallMovement {
                dir_vector: BALL_STARTING_DIR_VEC,
                speed: BALL_STARTING_SPEED,
            },
            collider: BallCollider {
                radius: BALL_RADIUS,
            },
            name: Name::new("Ball"),
            material: MaterialMesh2dBundle::default(),
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut ball_mesh: ResMut<Assets<Mesh>>,
    mut ball_material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        BallBundle {
            material: MaterialMesh2dBundle {
                mesh: ball_mesh.add(shape::Circle::new(BALL_RADIUS).into()).into(),
                material: ball_material.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(BALL_STARTING_POS)
                    .looking_at(BALL_STARTING_DIR_VEC, Vec3::Z),
                ..default()
            },
            ..default()
        }, // Ball,
           // MaterialMesh2dBundle{
           //     mesh: ball_mesh.add(shape::Circle::new(BALL_RADIUS).into()).into(),
           //     material: ball_material.add(ColorMaterial::from(BALL_COLOR)),
           //     transform: Transform::from_translation(BALL_STARTING_POS).looking_to(direction, up),
           //     ..default()
           // },
           // Ball{
           //     x_mag: BALL_STARTING_X_MAG,
           //     y_mag: BALL_STARTING_Y_MAG,
           //     speed: BALL_STARTING_SPEED,
           //     radius: BALL_RADIUS,
           // },
           // Name::new("Ball")
           // )
    );
}

// fn ball_movement(
//     mut ball_query: Query<(&GlobalTransform, &mut Transform, &mut Ball)>,
//     mut snake_query: Query<(&GlobalTransform, &SnakeHead)>,
//     time: Res<Time>,
//     bounding_box: Res<BoundingBox>,
// ) {
//     let (ball_global_transform, mut ball_transform , mut ball) = ball_query.single_mut();
//     let (snake_head_transform, snake_head) = snake_query.single();

//     ball_transform.translation.x += time.delta_seconds() * ball.x_mag * ball.speed;
//     ball_transform.translation.y += time.delta_seconds() * ball.y_mag * ball.speed;

//     if ball_transform.translation.x - ball.radius <= bounding_box.left_side ||
//         ball_transform.translation.x + ball.radius >= bounding_box.right_side {
//             info!("Collision ouccured on x axis at: x: ${:?}, y:{:?}", ball_transform.translation.x, ball_transform.translation.y);
//             ball.x_mag *= -1.;
//         }

//     if ball_transform.translation.y - ball.radius <= bounding_box.top_side ||
//         ball_transform.translation.y + ball.radius >= bounding_box.bottom_side {
//             info!("Collision ouccured on x axis at: x: ${:?}, y:{:?}", ball_transform.translation.x, ball_transform.translation.y);
//             ball.y_mag *= -1.;
//         }

//     if ball_global_transform.translation().distance(snake_head_transform.translation()) <= ball.radius + snake_head.radius {
//         ball_transform.
//     }
// }
