use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component, Debug)]
pub struct SnakeHead
{
    pub radius: f32,
}

#[derive(Component, Debug)]
pub struct SnakeBody {

}

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, (
                snake_movement,
            //     // snake_colision,
            ));
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut snake_mesh: ResMut<Assets<Mesh>>,
    mut snake_material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle{
            mesh: snake_mesh.add(shape::Circle::new(15.).into()).into(),
            material: snake_material.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(-40., 0., 0.)),
            ..default()
        },
        SnakeHead {
            radius: 15.,
        },
        Name::new("Snake")
        )
    );
}

fn snake_movement(
    mut snake_head_q: Query<(&mut Transform, &mut SnakeHead)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut snake_head_transform, mut snake_head) = snake_head_q.single_mut();

    if input.pressed(KeyCode::A) {
        snake_head_transform.rotate_local_z(3. * time.delta_seconds());
    } else if input.pressed(KeyCode::D) {

        snake_head_transform.rotate_local_z(-3. * time.delta_seconds());
    }
    let snake_dir = snake_head_transform.left();
    snake_head_transform.translation += snake_dir * time.delta_seconds() * 200.; 

}