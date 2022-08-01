use bevy::prelude::*;

use crate::{player::Laser, shared::AnimationTimer};

#[derive(Copy, Clone)]
pub(crate) enum BugMovement {
    Left,
    Right,
    Down { n: f32, next_left: bool },
}

#[derive(Component)]
pub(crate) struct Bug {
    pub(crate) movement: BugMovement,
}

pub(crate) fn bug_movement(mut query: Query<(&mut Bug, &mut Transform)>) {
    for (mut bug, mut trans) in query.iter_mut() {
        let mut new_movement = bug.movement;
        match bug.movement {
            BugMovement::Left => {
                trans.translation.x -= 2.0;
                if trans.translation.x < -300.0 {
                    new_movement = BugMovement::Down {
                        n: 12.0,
                        next_left: false,
                    };
                }
            }
            BugMovement::Right => {
                trans.translation.x += 2.0;
                if trans.translation.x > 300.0 {
                    new_movement = BugMovement::Down {
                        n: 12.0,
                        next_left: true,
                    };
                }
            }
            BugMovement::Down { n, next_left } => {
                trans.translation.y -= 2.0;
                new_movement = BugMovement::Down {
                    n: n - 1.0,
                    next_left,
                };
                if n < 1.0 {
                    new_movement = if next_left {
                        BugMovement::Left
                    } else {
                        BugMovement::Right
                    };
                }
            }
        }
        bug.movement = new_movement;
    }
}

pub(crate) fn bug_zapper(
    laser_query: Query<(Entity, &Laser, &Transform)>,
    collider_query: Query<(Entity, &Bug, &Transform)>,
    mut commands: Commands,
) {
    for (entity, _, trans) in laser_query.iter() {
        let laser_pos = Vec2::new(trans.translation.x, trans.translation.y);
        for (bug_entity, _, bug_transform) in collider_query.iter() {
            let bug_pos = Vec2::new(bug_transform.translation.x, bug_transform.translation.y);

            if bug_pos.distance(laser_pos) < 24.0 {
                commands.entity(bug_entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn spawn_bugs(mut commands: Commands, alan_handle: Handle<TextureAtlas>) {
    // Spawn rows of enemies
    for bug_row in 0..4 {
        let y = 200.0 - (bug_row as f32 * 30.0);
        for bug_col in 0..20 {
            let x = -300.0 + (bug_col as f32 * 30.0);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: alan_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..Default::default()
                })
                .insert(Bug {
                    movement: if bug_row % 2 == 0 {
                        BugMovement::Left
                    } else {
                        BugMovement::Right
                    },
                })
                .insert(AnimationTimer(Timer::from_seconds(0.5, true)));
        }
    }
}
