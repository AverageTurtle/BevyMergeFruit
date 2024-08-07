use bevy::{prelude::*, window::*, audio::Volume };
use bevy_xpbd_2d::{components::{RigidBody, Collider, CollidingEntities, Restitution, ColliderDensity, LinearDamping, LinearVelocity, AngularDamping}, resources::Gravity };

use crate::fruit::*;

#[derive(Resource)]
pub struct GameState {
    pub score: i32,
    pub current_fruit: FruitTypes,
    pub next_fruit: FruitTypes,
    pub game_over: bool,
    pub last_drop_time: f32
}

#[derive(Resource)]
pub struct MergeSound(Handle<AudioSource>);

#[derive(Resource)]
pub struct SpawnSound(Handle<AudioSource>);

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
        .insert_resource(Gravity(Vec2::NEG_Y * 4000.0))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_fruit)
        .add_systems(FixedUpdate, merge_fruit)
        .add_systems(FixedUpdate, stabilize)
        .add_systems(PostUpdate, game_over)
        .add_systems(Update, reset_game)
        .add_systems(Update, score_text_update)
        .add_systems(Update, grow_fruit)
        .add_systems(Update, preview_next_fruit)
        ;
    }
}

#[derive(Component)]
struct ScoreText;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_state: Res<GameState>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let merge_sound = asset_server.load("sfx/merge.ogg");
    commands.insert_resource(MergeSound(merge_sound));

    let spawn_sound = asset_server.load("sfx/spawn.ogg");
    commands.insert_resource(SpawnSound(spawn_sound));

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
        Collider::cuboid(700.0, 10.0),
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
                color: Color::rgba(0.8, 0.2, 0.2, 0.0),
                custom_size: Some(Vec2::new(700.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 165., 0.025),
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

    // Next Fruit
    commands.spawn((
        TextBundle::from_sections([ TextSection::new("Next", TextStyle { font: font.clone(), font_size: 35.0, color: Color::WHITE }) ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(404.0),
            ..default()
        }),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/ui_next_fruit.png"),
        transform: Transform::from_xyz(-350.0+460.0, 450.0-90.0, 0.0),
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..default()
            },

            texture: asset_server.load(FruitType::from(FruitTypes::Blueberry).texture),
            transform: Transform::from_xyz(-350.0+460.0, 450.0-90.0, 0.0),
            ..default()
        },
        NextFruitPreview { fruit_type: FruitTypes::Blueberry }
    ));

    // Score
    // Score label
    commands.spawn((
        TextBundle::from_sections([ TextSection::new("Score", TextStyle { font: font.clone(), font_size: 35.0, color: Color::WHITE }) ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/ui_score.png"),
        transform: Transform::from_xyz(-350.0+186.0, 450.0-90.0, 0.0),
        ..default()
    });

    // Score display
    commands.spawn((
        TextBundle::from_sections([ TextSection::from_style(TextStyle { font: font.clone(), font_size: 102.0, color: Color::BLACK }) ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(36.0),
            left: Val::Px(32.0),
            ..default()
        }),
        ScoreText
    ));
}
fn score_text_update(game_state : Res<GameState>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        let score = game_state.score;
        text.sections[0].value = format!("{score}");
        
    }
}

pub fn preview_next_fruit(game_state : Res<GameState>, asset_server: Res<AssetServer>, mut query: Query<(&mut NextFruitPreview, &mut Handle<Image>)>) {
    for (mut next, mut texture) in &mut query {
        if game_state.next_fruit != next.fruit_type {
            next.fruit_type = game_state.next_fruit;
            *texture = asset_server.load(FruitType::from(game_state.next_fruit).texture);
        }
    }
    
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
                game_state.score = 0;
            },
            _ => return,
        }
    }
}

pub fn game_over(mut commands: Commands, mut fruits: Query<(&Transform, &mut Fruit)>, time: Res<Time>, mut game_state: ResMut<GameState>) {
    for (transform, mut fruit) in &mut fruits {
       if transform.translation.y > 180. - ((fruit.size / 2.0)-16.0){
            if time.elapsed_seconds()-fruit.create_time >= 0.5 {
                fruit.death_time += time.delta_seconds();
                if fruit.death_time > 0.0 {
                    commands.insert_resource(ClearColor(Color::hex("#de8383").unwrap()));
                    game_state.game_over = true;
                }
            }
       }
    }
}

pub fn grow_fruit(time: Res<Time>, mut fruits: Query<(&mut Transform, &mut Fruit)>) {
    for (mut transform, mut fruit) in &mut fruits {
        fruit.size += 300.0 * time.delta_seconds();
        if fruit.size > fruit.target_size {
            fruit.size = fruit.target_size;
        }
        transform.scale = Vec3::new(fruit.size, fruit.size, 1.0);

    }
}

pub fn spawn_fruit(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>, sound: Res<SpawnSound>,
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
            preview_fruit_transform.translation = Vec3::new(x.clamp(-300.0+offset, 300.0-offset), 235.0, 0.5);

            if cursor_world_pos.y > 300.0 {
                cursor_above_limit = true
            }
    }

    if  cursor_above_limit == false && game_state.game_over == false && mouse.just_pressed(MouseButton::Left) && time.elapsed_seconds()-game_state.last_drop_time > 0.5 {
        let fruit = FruitType::from(game_state.current_fruit);
        commands.spawn((
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::new(0.0),
            ColliderDensity(7.0),
            LinearDamping(2.0),
            AngularDamping(0.1),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                texture: asset_server.load(fruit.texture),
                transform: Transform::from_xyz(preview_fruit_transform.translation.x, preview_fruit_transform.translation.y, 1.0).with_scale(Vec3::new(16.0, 16.0, 1.0)),
                ..default()
            },
            Fruit { fruit_type: game_state.current_fruit, create_time: time.elapsed_seconds(), death_time: -4.0, size: 16.0, target_size: fruit.size }
        ));
        game_state.score += fruit.value;

        game_state.current_fruit = game_state.next_fruit;
        game_state.next_fruit = get_random_fruit_type();
        game_state.last_drop_time = time.elapsed_seconds();

        let prev_fruit = FruitType::from(game_state.current_fruit);
        preview_fruit_transform.scale = Vec3::new(prev_fruit.size, prev_fruit.size, 1.0);
        *preview_fruit_texture = asset_server.load(prev_fruit.texture);

        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new_relative(0.25)).with_speed(0.9)
        });
    }
}

pub fn stabilize(mut query: Query<(&mut LinearVelocity, &Transform), With<Fruit>>) {
    for (mut velocity, transform) in &mut query {
        if velocity.y > 0.0 {
            velocity.y -= 0.1
        }

        if transform.translation.y > 178. && velocity.y > 0.0 {
            velocity.y = -0.01;
        }
    }
}
pub fn merge_fruit(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>, mut game_state: ResMut<GameState>, sound: Res<MergeSound>,
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
                            let pos;
                            if transform.translation.y < transform2.translation.y {
                                pos = transform.translation
                            } else {
                                pos = transform2.translation
                            }
                            let new_fruit = FruitType::from(new_fruit_type);

                            commands.spawn((
                                RigidBody::Dynamic,
                                Collider::ball(0.5),
                                Restitution::new(0.0),
                                ColliderDensity(7.0),
                                LinearDamping(2.0),
                                AngularDamping(0.01),
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::rgb(1.0, 1.0, 1.0),
                                        custom_size: Some(Vec2::new(1.0, 1.0)),
                                        ..default()
                                    },
                                    texture: asset_server.load(new_fruit.texture),
                                    transform: Transform::from_xyz(pos.x, pos.y, 1.0).with_scale(Vec3::new(16.0, 16.0, 1.0)),
                                    ..default()
                                },
                                Fruit { fruit_type: new_fruit_type, create_time: time.elapsed_seconds(), death_time: -1.0, size: 16.0, target_size: new_fruit.size }
                            ));
                            game_state.score += new_fruit.value;

                            commands.spawn(AudioBundle {
                                source: sound.0.clone(),
                                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new_relative(0.5))
                            });
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