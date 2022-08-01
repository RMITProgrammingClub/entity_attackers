use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player {
    pub delta_x: f32,
    pub next_shot: f64,
    pub shots_left: f32,
}

#[derive(Component, Default)]
pub struct CooldownBar {}

pub fn spawn_player(commands: &mut Commands, texture_atlas_handle: Handle<TextureAtlas>) {
    // Spawn the player
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -220.0, 0.0)),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(Player::default());

    // spawn the cooldown bar
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(100.0, 5.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -220.0, 0.0)),
            ..Default::default()
        })
        .insert(CooldownBar {});
}

#[derive(Component)]
pub struct Laser;

pub fn player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Player, &mut Transform, &Handle<TextureAtlas>)>,
        Query<(&mut CooldownBar, &mut Transform)>,
    )>,
) {
    const ACCELERATION: f32 = 1.0;
    const MAX_VELOCITY: f32 = 16.0;
    let mut player_c = (Player::default(), Transform::default());
    for (mut player, mut trans, atlas_handle) in set.p0().iter_mut() {
        let mut firing = false;

        if keyboard_input.pressed(KeyCode::Left) {
            player.delta_x -= ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.delta_x += ACCELERATION;
        }
        let time_now = time.seconds_since_startup();
        if player.shots_left <= 0.0 && player.next_shot < time_now {
            player.shots_left = 7.0;
        }
        if keyboard_input.pressed(KeyCode::Space)
            && player.next_shot < time_now
            && player.shots_left > 0.0
        {
            firing = true;
            player.next_shot = time_now + 0.2;
            player.shots_left -= 1.0;
            if player.shots_left <= 0.0 {
                player.next_shot = time_now + 2.0;
            }
        }

        // Apply movement deltas
        player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        trans.translation.x += player.delta_x;
        trans.translation.x = trans.translation.x.clamp(-320.0, 320.0);

        // Decelerate
        player.delta_x *= 0.75;

        if firing {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        trans.translation.x,
                        trans.translation.y + 0.0,
                        0.0,
                    )),
                    sprite: TextureAtlasSprite::new(2),
                    ..Default::default()
                })
                .insert(Laser {});
        }
        player_c = (player.clone(), trans.clone());
    }
    // Update the cooldown bar;
    for (_, mut cd_trans) in set.p1().iter_mut() {
        cd_trans.scale.x = player_c.0.shots_left / 7.0;
        cd_trans.translation.x = player_c.1.translation.x;
    }
}

pub fn laser_movement(mut query: Query<(Entity, &Laser, &mut Transform)>, mut commands: Commands) {
    for (entity, _, mut trans) in query.iter_mut() {
        trans.translation += Vec3::new(0.0, 4.0, 0.0);

        if trans.translation.y > 240.0 {
            commands.entity(entity).despawn();
        }
    }
}
