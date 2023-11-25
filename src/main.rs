use bevy::{prelude::*, window::*};
use bevy_xpbd_2d::prelude::*;
use game::GamePlugin;

mod game;
mod fruit;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(        
            WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Fruit Merge".into(),
                resolution: WindowResolution::new(700., 900.),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: false,
                enabled_buttons: EnabledButtons { minimize: true, maximize: false, close: true },
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
            }),
            PhysicsPlugins::default()
        ))
        .insert_resource(ClearColor(Color::hex("#ded083").unwrap()))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(GamePlugin)
        .run()
}