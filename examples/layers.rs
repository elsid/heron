use bevy::prelude::*;

use heron::*;

// ANCHOR: layer-enum
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
}
// ANCHOR_END: layer-enum

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0))) // Define the gravity
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let size = Vec2::new(1000.0, 50.0);
    let mut ground_entity = commands.spawn();
    ground_entity
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        });

    // ANCHOR: layer-component-world
    ground_entity.insert(
        CollisionLayers::none()
            .with_group(Layer::World)
            .with_mask(Layer::Player),
    );
    // ANCHOR_END: layer-component-world

    let size = Vec2::new(30.0, 30.0);
    let mut player_entity = commands.spawn();
    player_entity
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        });

    // ANCHOR: layer-component-player
    player_entity.insert(CollisionLayers::new(Layer::Player, Layer::World));
    // ANCHOR_END: layer-component-player
}
