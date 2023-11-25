use bevy::{prelude::*, window::*};
use bevy_xpbd_2d::{components::{RigidBody, Collider, CollidingEntities, Restitution}, resources::Gravity};

use crate::fruit::*;

#[derive(Resource)]
pub struct GameState {
    pub score: i32,
    pub current_fruit: FruitTypes,
    pub next_fruit: FruitTypes,
    pub game_over: bool,
    pub last_drop_time: f32
}

#[derive(Component)]
pub struct ResetButton;

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            current_fruit: get_random_fruit_type(),
            next_fruit: get_random_fruit_type(),
            game_over: false,
            last_drop_time: 0.0
        }
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Gravity(Vec2::NEG_Y * 800.0))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_fruit)
        .add_systems(Update, merge_fruit)
        .add_systems(Update, game_over)
        .add_systems(Update, reset_game)
        ;
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_state: Res<GameState>) {
    commands.spawn(Camera2dBundle {
        ..default()
    });

    commands.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(148.0),
                height: Val::Px(148.0),
                top: Val::Px(5.0),
                left: Val::Px(545.0),
                ..default()
            },
            image: UiImage::new(asset_server.load("textures/reset_button.png")),
            ..default()
        }, ResetButton));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/game_board.png"),
        transform: Transform::from_xyz(0.0, -130.0, 0.0),
        ..default()
    });

    //TODO WHY DOES IT BREAK WHEN I REMOVE THE SPRITE BUNDLE?? EVEN IF I KEEP THE TRANSFORM AHHHH I WANT TO DIE
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 900.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 900.0)),
                ..default()
            },
            transform: Transform::from_xyz(-325.0, 0.0, -0.1),
            ..default()
        }
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 900.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 900.0)),
                ..default()
            },
            transform: Transform::from_xyz(325.0, 0.0, -0.1),
            ..default()
        }
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(700.0, 20.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(700.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -440.0, -0.05),
            ..default()
        }
    ));

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(700.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 180., -0.025),
            ..default()
        }
    );

    // Preview Fruit
    let prev_fruit = FruitType::from(game_state.current_fruit);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                custom_size: Some(Vec2::new(1.0,1.0)),
                ..default()
            },
            texture: asset_server.load(prev_fruit.texture),
            transform: Transform::from_xyz(0.0, 220.0, 0.1).with_scale(Vec3::new(prev_fruit.size, prev_fruit.size, 1.0)),
            ..default()
        },
        PreviewFruit
    ));
}

pub fn reset_game(
    interaction_query: Query<&Interaction,(Changed<Interaction>, With<Button>, With<ResetButton>),>,
    mut commands: Commands, fruit_entities: Query<Entity, With<Fruit>>, mut game_state: ResMut<GameState>) 
{
    for interaction in &interaction_query {
        match *interaction  {
            Interaction::Pressed => {
                for entity in &fruit_entities {
                    commands.entity(entity).despawn();
                }
                commands.insert_resource(ClearColor(Color::hex("#ded083").unwrap()));
                game_state.game_over = false;
            },
            _ => return,
        }
    }
}

pub fn game_over(mut commands: Commands, fruits: Query<(&Transform, &Fruit)>, time: Res<Time>, mut game_state: ResMut<GameState>) {
    for (transform, fruit) in &fruits {
       if transform.translation.y > 180. {
            if time.elapsed_seconds()-fruit.create_time >= 3.0 {
                commands.insert_resource(ClearColor(Color::hex("#de8383").unwrap()));
                game_state.score = 0;
                game_state.game_over = true;
            }
       }
    }
}

pub fn spawn_fruit(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>,
    mouse: Res<Input<MouseButton>>, mut game_state: ResMut<GameState>,
    windows: Query<&Window, With<PrimaryWindow>>, camera: Query<(&Camera, &GlobalTransform)>,
    mut preview_fruit: Query<(&mut Transform, &mut Handle<Image>), With<PreviewFruit>>,
    mut _next_fruit_preview: Query<&mut Sprite, With<NextFruitPreview>>
    
) {

    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let (mut preview_fruit_transform, mut preview_fruit_texture) = preview_fruit.single_mut();
    //let mut sprite = preview_fruit_sprite.single_mut();

    let mut cursor_above_limit = false;
    if let Some(cursor_world_pos) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let x: f32 = cursor_world_pos.x;
            let offset: f32 = FruitType::from(game_state.current_fruit).size/2.;
            preview_fruit_transform.translation = Vec3::new(x.clamp(-300.0+offset, 300.0-offset), 220.0, 0.5);

            if cursor_world_pos.y > 300.0 {
                cursor_above_limit = true
            }
    }

    if  cursor_above_limit == false && game_state.game_over == false && mouse.just_pressed(MouseButton::Left) && time.elapsed_seconds()-game_state.last_drop_time > 0.5 {
        let fruit = FruitType::from(game_state.current_fruit);
        commands.spawn((
            RigidBody::Dynamic,
            Collider::ball(fruit.size/2.),
            Restitution::new(0.0),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(fruit.size, fruit.size)),
                    ..default()
                },
                texture: asset_server.load(fruit.texture),
                transform: Transform::from_xyz(preview_fruit_transform.translation.x, preview_fruit_transform.translation.y, 1.0),
                ..default()
            },
            Fruit { fruit_type: game_state.current_fruit, create_time: time.elapsed_seconds() }
        ));
        game_state.score += fruit.value;

        let score = game_state.score;
        println!("{score}");

        game_state.current_fruit = game_state.next_fruit;
        game_state.next_fruit = get_random_fruit_type();
        game_state.last_drop_time = time.elapsed_seconds();

        let prev_fruit = FruitType::from(game_state.current_fruit);
        preview_fruit_transform.scale = Vec3::new(prev_fruit.size, prev_fruit.size, 1.0);
        *preview_fruit_texture = asset_server.load(prev_fruit.texture);
    }
}

pub fn merge_fruit(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>, mut game_state: ResMut<GameState>,
    query: Query<(Entity, &Transform, &Fruit, &CollidingEntities)>) {
    //TODO fix this... this is so janky omg
    //the amount of indents makes me want to cry
    for (entity, transform, fruit, colliding_entities) in &query {
        for col_entity in colliding_entities.iter() {
            for (entity2, transform2, fruit2, _) in &query {
                if col_entity.index() == entity2.index()  {
                    if fruit.fruit_type == fruit2.fruit_type {

                        let old_fruit = FruitType::from(fruit.fruit_type);

                        if let Some(new_fruit_type) = old_fruit.next_fruit {
                            let pos = (transform.translation+transform2.translation)/2.;
                            let new_fruit = FruitType::from(new_fruit_type);
                            commands.spawn((
                                RigidBody::Dynamic,
                                Collider::ball(new_fruit.size/2.),
                                Restitution::new(0.0),
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::rgb(1.0, 1.0, 1.0),
                                        custom_size: Some(Vec2::new(new_fruit.size, new_fruit.size)),
                                        ..default()
                                    },
                                    texture: asset_server.load(new_fruit.texture),
                                    transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                                    ..default()
                                },
                                Fruit { fruit_type: new_fruit_type, create_time: time.elapsed_seconds() }
                            ));

                            game_state.score += new_fruit.value;
                            let score = game_state.score;
                            println!("{score}");
                        }

                        commands.entity(entity).despawn();
                        commands.entity(entity2).despawn();
                        return;
                    }
                 }
            }
        }
    }
}