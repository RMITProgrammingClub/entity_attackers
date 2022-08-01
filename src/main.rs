use bevy::prelude::*;
use bug::spawn_bugs;
use player::spawn_player;
use shared::AnimationTimer;

mod bug;
mod player;
mod shared;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Entity Attackers".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player::player)
        .add_system(player::laser_movement)
        .add_system(bug::bug_movement)
        .add_system(bug::bug_zapper)
        .add_system(animate_sprite)
        .run();
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Setup animated alan sprite sheet
    let alan_texture = asset_server.load("sprites/Alan (16 x 16).png");
    let alan_atlas = TextureAtlas::from_grid(alan_texture, Vec2::new(16.0, 16.0), 6, 1);
    let alan_handle = texture_atlases.add(alan_atlas);

    commands.spawn_bundle(Camera2dBundle::default());

    spawn_player(&mut commands, texture_atlas_handle);

    spawn_bugs(commands, alan_handle);
}
