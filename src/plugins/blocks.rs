use std::{time::Duration, f32::consts::PI};

use rand::prelude::*;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::{GameState, plugins::map::{ROWS, COLUMNS}};

use super::{map::{Row, Column, BLOCK_SIZE}, utils::center};

enum BlockType {
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock,
}

#[derive(Component)]
struct BlockParent;

#[derive(Component)]
struct FallingBlock {
    block_type: BlockType,
}

impl FallingBlock {
    fn spawn(piece: BlockType, commands: &mut Commands) -> Option<Entity> {
        let total_block_size = BLOCK_SIZE + 5.; // padding
        match piece {
            BlockType::TBlock => {
                let blocks_template = [[0., 0., 0.],[0., 1., 0.],[1., 1., 1.]];
                let (rows, columns) = (3, 3);
                Some(commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK,
                            custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0., 0., 20.),
                            rotation: Quat::from_rotation_z(PI),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                )
                .with_children(|parent| {
                    for (y, row) in blocks_template.iter().enumerate() {
                        for (x, column) in row.iter().enumerate() {
                            if column != &0. {
                                let pos = center(x as i32, y as i32, total_block_size, columns, rows);
                                parent.spawn_bundle(SpriteBundle {
                                    sprite: Sprite{
                                        color: Color::GREEN,
                                        custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(pos.x, pos.y - total_block_size/2., pos.z)),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                })
                    .insert(BlockParent)
                    .id())
            },
            _=> {
                None
            }
        }
    }
}

#[derive(Component)]
struct SpawningTimer {
    timer: Timer,
}

#[derive(Component)]
struct FallingTimer {
    normal_timer: Timer,
    fast_timer: Timer,
}

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Playing, setup_falling_timer)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(spawn_block)
                    .into()
            );
    }
}

fn setup_falling_timer(mut commands: Commands) {
    commands
        .insert_resource(
            FallingTimer{
                normal_timer: Timer::new(Duration::from_secs(1), true),
                fast_timer: Timer::new(Duration::from_millis(500), true)
            }
        );
    commands
        .insert_resource(
            SpawningTimer {
                timer: Timer::new(Duration::from_secs(3), true)
            }
        );
}

fn spawn_block(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawningTimer>,
    time: Res<Time>,
    blocks_query: Query<(Entity, &Row, &Column)>,
){
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        println!("finished!");
        let block_start_row = rand::thread_rng().gen_range(0..ROWS);
        let block_start_column = rand::thread_rng().gen_range(0..COLUMNS);
        for (block_e, block_row, block_column) in blocks_query.iter() {
            if block_column.column_number == block_start_column && 
                block_row.row_number == 1 {
                    let block = FallingBlock::spawn(BlockType::TBlock, &mut commands).unwrap();
                    commands.entity(block_e)
                        .add_child(block);
            }
        }
    }
    
}
